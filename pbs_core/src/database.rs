use std::hash::{Hash, Hasher};

use crate::{Error, Result};
use rusqlite::{
    types::{FromSql, FromSqlResult, ToSqlOutput, Value, ValueRef},
    Connection, ToSql,
};
use serde::{Deserialize, Serialize};

pub struct Database(Connection);

impl std::ops::Deref for Database {
    type Target = Connection;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// ==================================================================
// ItemMaturity
// ==================================================================

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ItemMaturity {
    InProgress,
    Released,
    Obsolete,
}

impl std::fmt::Display for ItemMaturity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let maturity = match self {
            ItemMaturity::InProgress => "In progress...",
            ItemMaturity::Released => "Released",
            ItemMaturity::Obsolete => "Obsolete",
        };
        write!(f, "{maturity}")
    }
}

const DB_MATURITY_IN_PROGRESS: i64 = 0;
const DB_MATURITY_RELEASED: i64 = 1;
const DB_MATURITY_OBSOLETE: i64 = 2;

impl FromSql for ItemMaturity {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_i64()? {
            DB_MATURITY_IN_PROGRESS => Ok(ItemMaturity::InProgress),
            DB_MATURITY_RELEASED => Ok(ItemMaturity::Released),
            DB_MATURITY_OBSOLETE => Ok(ItemMaturity::Obsolete),
            _ => todo!("DB : Manage the maturity conversion"),
        }
    }
}

impl ToSql for ItemMaturity {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let value = match self {
            ItemMaturity::InProgress => DB_MATURITY_IN_PROGRESS,
            ItemMaturity::Released => DB_MATURITY_RELEASED,
            ItemMaturity::Obsolete => DB_MATURITY_OBSOLETE,
        };
        Ok(ToSqlOutput::Owned(Value::Integer(value)))
    }
}

// ==================================================================
// ItemType
// ==================================================================

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Strategy {
    Make,
    Buy,
}

impl std::fmt::Display for Strategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let maturity = match self {
            Strategy::Make => "Make",
            Strategy::Buy => "Buy",
        };
        write!(f, "{maturity}")
    }
}

const DB_ITEM_MAKE: i64 = 0;
const DB_ITEM_BUY: i64 = 1;

impl FromSql for Strategy {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_i64()? {
            DB_ITEM_MAKE => Ok(Strategy::Make),
            DB_ITEM_BUY => Ok(Strategy::Buy),
            _ => todo!("DB : Manage the item strategy conversion"),
        }
    }
}

impl ToSql for Strategy {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let value = match self {
            Strategy::Make => DB_ITEM_MAKE,
            Strategy::Buy => DB_ITEM_BUY,
        };
        Ok(ToSqlOutput::Owned(Value::Integer(value)))
    }
}

// ==================================================================
// Item
// ==================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    id: i64,
    pn: String,
    version: usize,
    name: String,
    maturity: ItemMaturity,
    strategy: Strategy,
}

impl Item {
    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn pn(&self) -> &str {
        &self.pn
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> usize {
        self.version
    }

    pub fn maturity(&self) -> ItemMaturity {
        self.maturity
    }

    pub fn strategy(&self) -> Strategy {
        self.strategy
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.strategy {
            Strategy::Make => write!(
                f,
                "{id} : MAKE [{pn}-{version:03}] - \"{name}\" - {maturity}",
                id = self.id,
                pn = self.pn,
                name = self.name,
                version = self.version,
                maturity = self.maturity
            ),
            Strategy::Buy => write!(
                f,
                "{id} : BUY [{pn}] - \"{name}\" - {maturity}",
                id = self.id,
                pn = self.pn,
                name = self.name,
                maturity = self.maturity
            ),
        }
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Item {}

impl Hash for Item {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<'stmt> TryFrom<&rusqlite::Row<'stmt>> for Item {
    type Error = rusqlite::Error;
    fn try_from(value: &rusqlite::Row) -> std::result::Result<Self, Self::Error> {
        Ok(Item {
            id: value.get("id")?,
            pn: value.get("pn")?,
            name: value.get("name")?,
            version: value.get("version")?,
            maturity: value.get("maturity")?,
            strategy: value.get("strategy")?,
        })
    }
}

trait ErrConvert<T> {
    fn convert(self) -> Result<T>;
}

impl<T> ErrConvert<T> for rusqlite::Result<T> {
    fn convert(self) -> Result<T> {
        self.map_err(Error::DatabaseErr)
    }
}

impl Database {
    /// Open the store
    pub(crate) fn open(url: &str) -> Result<Self> {
        let conn = Connection::open(url).convert()?;
        for req in include_str!("db.sql").split(';').filter(|s| !s.is_empty()) {
            conn.execute(req, ()).convert()?;
        }
        Ok(Database(conn))
    }

    // Get a config value from database
    pub fn read_config(&self, key: &str) -> Result<String> {
        let mut stmt = self
            .prepare("SELECT value FROM config WHERE key = ?1")
            .convert()?;
        match stmt.query_row([key], |row| row.get("value")) {
            Ok(value) => Ok(value),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok("".to_string()),
            Err(e) => Err(e),
        }
        .convert()
    }

    // Set a config value in database
    pub fn write_config(&self, key: &str, value: &str) -> Result<()> {
        let mut stmt = self
            .prepare("REPLACE into config(key, value) VALUES(?1, ?2)")
            .convert()?;
        stmt.execute((key, value)).map(|_| ()).convert()
    }

    // Add a new item to the store
    pub(crate) fn insert_item(&self, pn: &str, name: &str, t: Strategy) -> Result<Item> {
        const DEFAULT_VERSION: usize = 1;
        const DEFAULT_MATURITY_MAKE: ItemMaturity = ItemMaturity::InProgress;
        const DEFAULT_MATURITY_BUY: ItemMaturity = ItemMaturity::Released;

        let maturity = match t {
            Strategy::Make => DEFAULT_MATURITY_MAKE,
            Strategy::Buy => DEFAULT_MATURITY_BUY,
        };

        self.execute(
            "INSERT INTO items(pn, name, version, maturity, strategy) VALUES(?1, ?2, ?3, ?4, ?5)",
            (pn, name, DEFAULT_VERSION, maturity, t),
        )
        .convert()?;
        let id = self.last_insert_rowid();
        self.item(id)
    }

    /// Retrive all [Item]s
    pub(crate) fn items(&self) -> Result<Vec<Item>> {
        let mut stmt = self.prepare("SELECT * FROM items").convert()?;
        let items = stmt
            .query_map([], |row| Item::try_from(row))
            .convert()?
            .filter_map(|i| i.ok())
            .collect::<Vec<_>>();
        Ok(items)
    }

    /// Update the item
    // pub(crate) fn update_item(&mut self, item: Item) -> Result<()> {
    //     if self
    //         .execute(
    //             "UPDATE items set pn=(?1), name=(?2) where id=(?3)",
    //             (&item.pn, &item.name, item.id),
    //         )
    //         .convert()?
    //         != 1
    //     {
    //         return Err(Error::DatabaseErr(rusqlite::Error::QueryReturnedNoRows));
    //     }
    //     Ok(())
    // }

    /// Get an `Item` by it's ID
    pub fn item(&self, id: i64) -> Result<Item> {
        let mut stmt = self
            .prepare("SELECT * FROM items WHERE id = ?1")
            .convert()?;
        stmt.query_row([id], |row| Item::try_from(row)).convert()
    }

    /// Add a child to an item
    pub(crate) fn add_child(
        &mut self,
        parent_id: i64,
        child_id: i64,
        quantity: usize,
    ) -> Result<()> {
        if self
            .execute(
                "INSERT INTO children (id_parent, id_child, quantity) VALUES(?1, ?2, ?3)",
                (parent_id, child_id, quantity),
            )
            .convert()?
            != 1
        {
            return Err(Error::DatabaseErr(rusqlite::Error::QueryReturnedNoRows));
        }
        Ok(())
    }

    /// Delete a child (and its quantity) from an item
    pub(crate) fn delete_child(&mut self, parent_id: i64, child_id: i64) -> Result<()> {
        if self
            .execute(
                "DELETE FROM children WHERE id_parent = ?1 and id_child = ?2",
                (parent_id, child_id),
            )
            .convert()?
            != 1
        {
            return Err(Error::DatabaseErr(rusqlite::Error::QueryReturnedNoRows));
        }
        Ok(())
    }

    /// Get children of an item
    pub(crate) fn children(&self, parent_id: i64) -> Result<Vec<(Item, usize)>> {
        let mut stmt = self
            .prepare("SELECT * FROM view_children WHERE id_parent = ?1")
            .convert()?;
        let items = stmt
            .query_map([parent_id], |row| {
                let item = Item::try_from(row)?;
                let quantity = row.get("quantity")?;
                Ok((item, quantity))
            })
            .convert()?
            .filter_map(|i| i.ok())
            .collect::<Vec<_>>();
        Ok(items)
    }

    ///
    pub(crate) fn where_used(&self, id: i64) -> Result<Vec<Item>> {
        let mut stmt = self
            .prepare("SELECT * FROM view_where_used WHERE id_child = ?1")
            .convert()?;
        let items = stmt
            .query_map([id], |row| Item::try_from(row))
            .convert()?
            .filter_map(|i| i.ok())
            .collect::<Vec<_>>();
        Ok(items)
    }

    /// Search for items match pn or name columns
    pub fn search(&self, pattern: &str) -> Result<Vec<Item>> {
        let mut stmt = self
            .prepare("SELECT * FROM items WHERE pn LIKE ?1 or name LIKE ?1 ORDER BY pn")
            .convert()?;
        let items = stmt
            .query_map([pattern], |row| Item::try_from(row))
            .convert()?
            .filter_map(|i| i.ok())
            .collect::<Vec<_>>();
        Ok(items)
    }

    /// Release an Item
    pub fn release(&mut self, id: i64) -> Result<Item> {
        if self
            .execute(
                "UPDATE items set maturity=(?1) where id=(?2)",
                (ItemMaturity::Released, id),
            )
            .convert()?
            != 1
        {
            return Err(Error::DatabaseErr(rusqlite::Error::QueryReturnedNoRows));
        }
        self.item(id)
    }

    /// Make an Item [ItemMaturity::Obsolete]
    pub fn make_obsolete(&mut self, id: i64) -> Result<Item> {
        if self
            .execute(
                "UPDATE items set maturity=(?1) where id=(?2)",
                (ItemMaturity::Obsolete, id),
            )
            .convert()?
            != 1
        {
            return Err(Error::DatabaseErr(rusqlite::Error::QueryReturnedNoRows));
        }
        self.item(id)
    }
}

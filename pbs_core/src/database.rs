use std::{
    fmt::Display,
    hash::{Hash, Hasher},
};

use crate::{Error, Result};
use rusqlite::{
    types::{FromSql, FromSqlResult, ToSqlOutput, Value, ValueRef},
    Connection, ToSql,
};

pub struct Database(Connection);

#[derive(Copy, Clone)]
pub enum ItemMaturity {
    InProgress = 0,
    Released = 1,
}

impl Display for ItemMaturity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let maturity = match self {
            ItemMaturity::InProgress => "In progress...",
            ItemMaturity::Released => "Released",
        };
        write!(f, "{maturity}")
    }
}

impl FromSql for ItemMaturity {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_i64()? {
            x if x == ItemMaturity::InProgress as i64 => Ok(ItemMaturity::InProgress),
            x if x == ItemMaturity::Released as i64 => Ok(ItemMaturity::Released),
            _ => todo!("DB : Manage the maturity conversion"),
        }
    }
}

impl ToSql for ItemMaturity {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(Value::Integer(*self as i64)))
    }
}

struct InnerItem {
    pn: String,
    name: String,
    maturity: ItemMaturity,
    version: usize,
}

impl InnerItem {
    fn new(pn: &str, name: &str) -> Self {
        InnerItem {
            pn: pn.to_string(),
            name: name.to_string(),
            version: 1,
            maturity: ItemMaturity::InProgress,
        }
    }
}

impl<'stmt> TryFrom<&rusqlite::Row<'stmt>> for InnerItem {
    type Error = rusqlite::Error;
    fn try_from(value: &rusqlite::Row) -> std::result::Result<Self, Self::Error> {
        Ok(InnerItem {
            pn: value.get("pn")?,
            name: value.get("name")?,
            version: value.get("version")?,
            maturity: value.get("maturity")?,
        })
    }
}

pub struct Item {
    _id: usize,
    inner: InnerItem,
}

impl Item {
    fn new(id: usize, inner: InnerItem) -> Self {
        Item { _id: id, inner }
    }

    pub fn pn(&self) -> &str {
        &self.inner.pn
    }

    pub fn name(&self) -> &str {
        &self.inner.name
    }

    pub fn version(&self) -> usize {
        self.inner.version
    }

    pub fn maturity(&self) -> ItemMaturity {
        self.inner.maturity
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{pn}-{version:03}] \"{name}\" - {maturity}",
            pn = self.pn(),
            name = self.name(),
            version = self.version(),
            maturity = self.maturity()
        )
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self._id == other._id
    }
}

impl Eq for Item {}

impl Hash for Item {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self._id.hash(state);
    }
}

impl<'stmt> TryFrom<&rusqlite::Row<'stmt>> for Item {
    type Error = rusqlite::Error;
    fn try_from(value: &rusqlite::Row) -> std::result::Result<Self, Self::Error> {
        Ok(Item {
            _id: value.get("id")?,
            inner: InnerItem::try_from(value)?,
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

const INIT_DB: [&str; 5] = [
    "PRAGMA foreign_keys = ON;",
    "CREATE TABLE IF NOT EXISTS items(
        id        INTEGER PRIMARY KEY,
        pn        TEXT,
        name      TEXT,
        maturity  INTEGER,
        version   INTEGER,
        UNIQUE(pn)
    );",
    "CREATE TABLE IF NOT EXISTS children(
        id_parent  INTEGER,
        id_child   INTEGER,
        quantity   INTEGER,
        FOREIGN KEY(id_parent) REFERENCES items(id),
        FOREIGN KEY(id_child) REFERENCES items(id)
    );",
    "CREATE VIEW IF NOT EXISTS view_children AS
        SELECT
            items.id, 
            items.pn, 
            items.name, 
            items.version,
            items.maturity,
            children.quantity,
            children.id_parent
        FROM items, children 
        WHERE children.id_child = items.id",
    "CREATE VIEW IF NOT EXISTS view_where_used AS
    SELECT
            children.id_parent as id,
            items.pn, 
            items.name,
            items.version,
            items.maturity,
            children.id_child
        FROM items, children 
        WHERE children.id_parent = items.id",
];

impl Database {
    /// Open the store
    pub fn open(url: &str) -> Result<Self> {
        let conn = Connection::open(url).convert()?;
        //TODO
        //let init_reqs = include_str!("db.sql");
        for req in INIT_DB {
            conn.execute(req, ()).convert()?;
        }

        Ok(Database(conn))
    }

    // Add a new item to the store
    pub fn insert_item(&self, pn: &str, name: &str) -> Result<Item> {
        let inner_item = InnerItem::new(pn, name);
        self.0
            .execute(
                "INSERT INTO items(pn, name, version, maturity) VALUES(?1, ?2, ?3, ?4)",
                (
                    &inner_item.pn,
                    &inner_item.name,
                    inner_item.version,
                    inner_item.maturity,
                ),
            )
            .convert()?;
        let id = self.0.last_insert_rowid();
        Ok(Item::new(id as usize, inner_item))
    }

    /// Retrive all [Item]s
    pub fn get_items(&self) -> Result<Vec<Item>> {
        let mut stmt = self.0.prepare("SELECT * FROM items").convert()?;
        let items = stmt
            .query_map([], |row| Item::try_from(row))
            .convert()?
            .filter_map(|i| i.ok())
            .collect::<Vec<_>>();
        Ok(items)
    }

    /// Update the item
    pub fn update_item(&mut self, item: Item) -> Result<()> {
        if self
            .0
            .execute(
                "UPDATE items set pn=(?1), name=(?2) where id=(?3)",
                (&item.pn(), &item.name(), item._id),
            )
            .convert()?
            != 1
        {
            return Err(Error::DatabaseErr(rusqlite::Error::QueryReturnedNoRows));
        }
        Ok(())
    }

    /// Get `Item` by it's PN
    ///
    /// WARNING : this function returns the 1st result (but there
    /// should be only 1 result)
    pub fn get_item_by_pn(&self, pn: &str) -> Result<Item> {
        let mut stmt = self
            .0
            .prepare("SELECT * FROM items WHERE pn = ?1")
            .convert()?;
        stmt.query_row([pn], |row| Item::try_from(row)).convert()
    }

    /// Add a child to an item
    pub fn add_child(&mut self, parent: &Item, child: &Item, quantity: usize) -> Result<()> {
        if self
            .0
            .execute(
                "INSERT INTO children (id_parent, id_child, quantity) VALUES(?1, ?2, ?3)",
                (parent._id, child._id, quantity),
            )
            .convert()?
            != 1
        {
            return Err(Error::DatabaseErr(rusqlite::Error::QueryReturnedNoRows));
        }
        Ok(())
    }

    /// Get children of an item
    pub fn get_children(&self, parent: &Item) -> Result<Vec<(Item, usize)>> {
        let mut stmt = self
            .0
            .prepare("SELECT * FROM view_children WHERE id_parent = ?1")
            .convert()?;
        let items = stmt
            .query_map([parent._id], |row| {
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
    pub fn where_used(&self, item: &Item) -> Result<Vec<Item>> {
        let mut stmt = self
            .0
            .prepare("SELECT * FROM view_where_used WHERE id_child = ?1")
            .convert()?;
        let items = stmt
            .query_map([item._id], |row| Item::try_from(row))
            .convert()?
            .filter_map(|i| i.ok())
            .collect::<Vec<_>>();
        Ok(items)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn init_database() {
        assert!(Database::open(":memory:").is_ok());
    }

    #[test]
    fn add_items() {
        let db = Database::open(":memory:").unwrap();
        assert!(db.insert_item("PN1", "NAME1").is_ok());
        let items = db.get_items();
        assert!(items.is_ok());
        let items = items.unwrap();
        assert_eq!(1, items.len());
    }

    #[test]
    fn add_childrens() {
        let mut db = Database::open(":memory:").unwrap();
        let item1 = db.insert_item("1", "PARENT").unwrap();
        let item2 = db.insert_item("11", "CHILD1").unwrap();
        let item3 = db.insert_item("12", "CHILD2").unwrap();
        db.add_child(&item1, &item2, 1).unwrap();
        db.add_child(&item1, &item3, 2).unwrap();
        let children = db.get_children(&item1).unwrap();
        assert_eq!(2, children.len());
    }

    #[test]
    fn add_same_pn() {
        let db = Database::open(":memory:").unwrap();
        let _ = db.insert_item("PN", "ITEM").unwrap();
        assert!(db.insert_item("PN", "ANOTHER").is_err());
    }
}

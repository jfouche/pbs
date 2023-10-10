use std::hash::{Hash, Hasher};

use crate::{Error, Result};
use rusqlite::{
    types::{FromSql, FromSqlResult, ToSqlOutput, Value, ValueRef},
    Connection, ToSql,
};
use serde::Serialize;

pub struct Database(Connection);

#[derive(Copy, Clone, Serialize)]
pub enum ItemMaturity {
    InProgress = 0,
    Released = 1,
}

impl std::fmt::Display for ItemMaturity {
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

#[derive(Serialize)]
pub struct Item {
    _id: usize,
    pn: String,
    name: String,
    maturity: ItemMaturity,
    version: usize,
}

impl Item {
    pub fn id(&self) -> usize {
        self._id
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
}

impl std::fmt::Display for Item {
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
            pn: value.get("pn")?,
            name: value.get("name")?,
            version: value.get("version")?,
            maturity: value.get("maturity")?,
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
    pub fn get_config(&self, key: &str) -> Result<String> {
        let mut stmt = self
            .0
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
    pub fn set_config(&self, key: &str, value: &str) -> Result<()> {
        let mut stmt = self
            .0
            .prepare("REPLACE into config(key, value) VALUES(?1, ?2)")
            .convert()?;
        stmt.execute((key, value)).map(|_| ()).convert()
    }

    // Add a new item to the store
    pub(crate) fn insert_item(&self, pn: &str, name: &str) -> Result<Item> {
        const DEFAULT_VERSION: usize = 1;
        const DEFAULT_MATURITY: ItemMaturity = ItemMaturity::InProgress;

        self.0
            .execute(
                "INSERT INTO items(pn, name, version, maturity) VALUES(?1, ?2, ?3, ?4)",
                (pn, name, DEFAULT_VERSION, DEFAULT_MATURITY),
            )
            .convert()?;
        let id = self.0.last_insert_rowid();
        self.get_item_by_id(id as usize)
    }

    /// Retrive all [Item]s
    pub(crate) fn get_items(&self) -> Result<Vec<Item>> {
        let mut stmt = self.0.prepare("SELECT * FROM items").convert()?;
        let items = stmt
            .query_map([], |row| Item::try_from(row))
            .convert()?
            .filter_map(|i| i.ok())
            .collect::<Vec<_>>();
        Ok(items)
    }

    /// Update the item
    pub(crate) fn update_item(&mut self, item: Item) -> Result<()> {
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

    /// Get `Item` by it's ID
    ///
    /// WARNING : this function returns the 1st result (but there
    /// should be only 1 result)
    pub fn get_item_by_id(&self, id: usize) -> Result<Item> {
        let mut stmt = self
            .0
            .prepare("SELECT * FROM items WHERE id = ?1")
            .convert()?;
        stmt.query_row([id], |row| Item::try_from(row)).convert()
    }

    /// Add a child to an item
    pub(crate) fn add_child(
        &mut self,
        parent_id: usize,
        child_id: usize,
        quantity: usize,
    ) -> Result<()> {
        if self
            .0
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

    /// Get children of an item
    pub(crate) fn get_children_by_parent_id(&self, parent_id: usize) -> Result<Vec<(Item, usize)>> {
        let mut stmt = self
            .0
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

    /// Get children of an item
    pub(crate) fn get_children(&self, parent: &Item) -> Result<Vec<(Item, usize)>> {
        self.get_children_by_parent_id(parent.id())
    }

    ///
    pub(crate) fn where_used(&self, item: &Item) -> Result<Vec<Item>> {
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

    /// Search for items match pn or name columns
    pub fn search(&self, pattern: &str) -> Result<Vec<Item>> {
        let mut stmt = self
            .0
            .prepare("SELECT * FROM items WHERE pn LIKE ?1 or name LIKE ?1 ORDER BY pn")
            .convert()?;
        let items = stmt
            .query_map([pattern], |row| Item::try_from(row))
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
        db.add_child(item1._id, item2._id, 1).unwrap();
        db.add_child(item1._id, item3._id, 2).unwrap();
        let children = db.get_children(&item1).unwrap();
        assert_eq!(2, children.len());

        // can't add an already existing child
        assert!(db.add_child(item1._id, item3._id, 2).is_err());
    }

    #[test]
    fn add_same_pn() {
        let db = Database::open(":memory:").unwrap();
        let _ = db.insert_item("PN", "ITEM").unwrap();
        assert!(db.insert_item("PN", "ANOTHER").is_err());
    }

    #[test]
    fn config() {
        let db = Database::open(":memory:").unwrap();
        assert_eq!("".to_string(), db.get_config("key").unwrap());
        assert!(db.set_config("key", "value").is_ok());
        assert_eq!("value".to_string(), db.get_config("key").unwrap());
        let _ = db.set_config("key", "value 2");
        assert_eq!("value 2", db.get_config("key").unwrap());
    }

    #[test]
    fn search() {
        let db = Database::open(":memory:").unwrap();
        db.insert_item("00000001", "FIRST ITEM").unwrap();
        db.insert_item("00000002", "SECOND ITEM").unwrap();
        db.insert_item("00000003", "THIRD THING").unwrap();
        db.insert_item("123.456", "EXTERNAL THING").unwrap();
        db.insert_item("123.003", "OTHER EXTERNAL THING").unwrap();
        db.insert_item("123.678", "THING 1003").unwrap();

        let items = db.search("%000%").unwrap();
        assert_eq!(3, items.len());
        assert_eq!("00000001", items.get(0).unwrap().pn());
        assert_eq!("00000002", items.get(1).unwrap().pn());
        assert_eq!("00000003", items.get(2).unwrap().pn());

        let items = db.search("%003%").unwrap();
        assert_eq!(3, items.len());
        assert_eq!("00000003", items.get(0).unwrap().pn());
        assert_eq!("123.003", items.get(1).unwrap().pn());
        assert_eq!("123.678", items.get(2).unwrap().pn());
    }
}

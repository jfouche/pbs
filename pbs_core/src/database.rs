use rusqlite::Connection;

pub struct Database(Connection);

#[derive(Debug)]
pub enum Error {
    DatabaseErr(rusqlite::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Item {
    _id: usize,
    pub pn: String,
    pub name: String,
}

/// TODO : use this trait
impl<'stmt> TryFrom<rusqlite::Row<'stmt>> for Item {
    type Error = Error;
    fn try_from(value: rusqlite::Row) -> std::result::Result<Self, Self::Error> {
        Ok(Item {
            _id: value.get("id").convert()?,
            pn: value.get("pn").convert()?,
            name: value.get("name").convert()?,
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

const INIT_DB: [&str; 4] = [
    "PRAGMA foreign_keys = ON;",
    "CREATE TABLE IF NOT EXISTS items(
        id   INTEGER PRIMARY KEY,
        pn   TEXT,
        name TEXT
    );",
    "CREATE TABLE IF NOT EXISTS children(
        id_parent INTEGER,
        id_child  INTEGER,
        quantity  INTEGER,
        FOREIGN KEY(id_parent) REFERENCES items(id),
        FOREIGN KEY(id_child) REFERENCES items(id)
    );",
    "CREATE VIEW IF NOT EXISTS view_children AS
        SELECT
            items.id, 
            items.pn, 
            items.name, 
            children.id_parent,
            children.quantity 
        FROM items, children 
        WHERE children.id_child = items.id",
];

impl Database {
    /// Open the store
    pub fn open(url: &str) -> Result<Self> {
        let conn = Connection::open(url).convert()?;
        for req in INIT_DB {
            conn.execute(req, ()).convert()?;
        }

        Ok(Database(conn))
    }

    // Add a new item to the store
    pub fn insert_item(&self, pn: &str, name: &str) -> Result<Item> {
        self.0
            .execute(
                "INSERT INTO items(pn, name) VALUES(?1, ?2)",
                [pn.to_string(), name.to_string()],
            )
            .convert()?;
        let id = self.0.last_insert_rowid();
        Ok(Item {
            _id: id as usize,
            pn: pn.to_string(),
            name: name.to_string(),
        })
    }

    pub fn get_items(&self) -> Result<Vec<Item>> {
        let mut stmt = self.0.prepare("SELECT id, pn, name FROM items").convert()?;
        let items = stmt
            .query_map([], |row| {
                Ok(Item {
                    _id: row.get(0)?,
                    pn: row.get(1)?,
                    name: row.get(2)?,
                })
            })
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
                (&item.pn, &item.name, item._id),
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
            .prepare("SELECT id, pn, name FROM items WHERE pn = ?1")
            .convert()?;
        let mut rows = stmt.query([pn]).convert()?;
        let row1 = rows.next().convert()?;
        let row1 = row1.ok_or(Error::DatabaseErr(rusqlite::Error::QueryReturnedNoRows))?;
        // TODO        let item: Result<Item> = row1.try_into();
        Ok(Item {
            _id: row1.get(0).convert()?,
            pn: row1.get(1).convert()?,
            name: row1.get(2).convert()?,
        })
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
            .prepare("SELECT id, pn, name, quantity FROM view_children WHERE id_parent = ?1")
            .convert()?;
        let items = stmt
            .query_map([parent._id], |row| {
                Ok((
                    Item {
                        _id: row.get(0)?,
                        pn: row.get(1)?,
                        name: row.get(2)?,
                    },
                    row.get(3)?,
                ))
            })
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
}

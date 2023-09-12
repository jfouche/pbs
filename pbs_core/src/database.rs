use rusqlite::*;

pub struct Database(Connection);

pub struct Item {
    _id: usize,
    pub pn: String,
    pub name: String,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    DatabaseErr(rusqlite::Error),
}

fn db_err(e: rusqlite::Error) -> Error {
    Error::DatabaseErr(e)
}

const INIT_TABLES: &str = "CREATE TABLE IF NOT EXISTS items(
    id   INTEGER PRIMARY KEY,
    pn   TEXT,
    name TEXT
);

CREATE TABLE IF NOT EXISTS children(
    id_parent INTEGER,
    id_child  INTEGER,
    quantity  INTEGER,
    FOREIGN KEY(id_parent) REFERENCES items(id),
    FOREIGN KEY(id_child) REFERENCES items(id)
);";

impl Database {
    /// Open the store
    pub fn open(url: &str) -> Result<Self> {
        let conn = Connection::open(url).map_err(db_err)?;
        conn.execute(INIT_TABLES, ()).map_err(db_err)?;
        Ok(Database(conn))
    }

    // Add a new item to the store
    pub fn insert_item(&self, pn: &str, name: &str) -> Result<Item> {
        self.0
            .execute(
                "INSERT INTO items(pn, name) VALUES(?1, ?2)",
                [pn.to_string(), name.to_string()],
            )
            .map_err(db_err)?;
        let id = self.0.last_insert_rowid();
        Ok(Item {
            _id: id as usize,
            pn: pn.to_string(),
            name: name.to_string(),
        })
    }

    pub fn get_items(&self) -> Result<Vec<Item>> {
        let mut stmt = self
            .0
            .prepare("SELECT id, pn, name FROM items")
            .map_err(db_err)?;
        let items = stmt
            .query_map([], |row| {
                Ok(Item {
                    _id: row.get(0)?,
                    pn: row.get(1)?,
                    name: row.get(2)?,
                })
            })
            .map_err(db_err)?
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
            .map_err(db_err)?
            != 1
        {
            return Err(Error::DatabaseErr(rusqlite::Error::QueryReturnedNoRows));
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    fn init_database() {
        let db = Database::open(":memory:");
        assert!(db.is_ok());
        let db = db.unwrap();
        assert!(db.insert_item("PN1", "NAME1").is_ok());
        let items = db.get_items();
        assert!(items.is_ok());
        let items = items.unwrap();
        assert_eq!(1, items.len());
    }
}

use rusqlite::*;

#[derive(Debug)]
pub enum Error {
    DatabaseErr(rusqlite::Error),
}

fn db_err(e: rusqlite::Error) -> Error {
    Error::DatabaseErr(e)
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

// trait ErrConverter<T> {
//     fn if_err(err: Error) -> Result<T>;
// }

// impl<T> ErrConverter<T> for rusqlite::Result<T> {
//     fn if_err(err: Error) -> Result<T> {
//         Result::Err(err)
//     }
// }

pub struct Item {
    _id: usize,
    pub pn: String,
    pub name: String,
}

pub struct Store {
    conn: Connection,
}

impl Store {
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

    /// Open the store
    pub fn open(url: &str) -> Result<Self> {
        let conn = Connection::open(url).map_err(db_err)?;
        conn.execute(Store::INIT_TABLES, ()).map_err(db_err)?;
        Ok(Store { conn })
    }

    // Add a new item to the store
    pub fn new_item(&self, pn: &str) -> Result<Item> {
        self.conn
            .execute("INSERT INTO items(pn) VALUES(?1)", [pn.to_string()])
            .map_err(db_err)?;
        let id = self.conn.last_insert_rowid();
        Ok(Item {
            _id: id as usize,
            pn: pn.to_string(),
            name: String::new(),
        })
    }

    /// Save the item
    pub fn save_item(&mut self, item: Item) -> Result<()> {
        if self
            .conn
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

    pub fn get_items(&self) -> Result<Vec<Item>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, pn, name, data FROM itms")
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
}

#[cfg(test)]
mod test {
    use super::*;

    const DB_URL: &str = ":memory:";

    // #[test]
    fn init_database() {
        std::fs::remove_file(DB_URL).unwrap();

        let store = Store::open(DB_URL);
        assert!(store.is_ok());
        let store = store.unwrap();
        assert!(store.new_item("PN1").is_ok());
        let items = store.get_items();
        assert!(items.is_ok());
        let items = items.unwrap();
        assert_eq!(1, items.len());
    }
}

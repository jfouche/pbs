use rusqlite::*;

pub struct Item {
    _id: usize,
    name: String,
}

pub struct Store {
    conn: Connection,
}

impl Store {
    const INIT_TABLES: &str = "CREATE TABLE IF NOT EXISTS items(
        id   INTEGER PRIMARY KEY,
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
        let conn = Connection::open(url)?;
        conn.execute(Store::INIT_TABLES, ())?;
        Ok(Store { conn })
    }

    // Add a new item to the store
    pub fn new_item(&self) -> Result<Item> {
        self.conn
            .execute("INSERT INTO items(name) VALUES(?1)", ["".to_string()])?;
        let id = self.conn.last_insert_rowid();
        Ok(Item {
            _id: id as usize,
            name: String::new(),
        })
    }

    pub fn get_items(&self) -> Result<Vec<Item>> {
        let mut stmt = self.conn.prepare("SELECT id, name, data FROM itms")?;
        let items = stmt
            .query_map([], |row| {
                Ok(Item {
                    _id: row.get(0)?,
                    name: row.get(1)?,
                })
            })?
            .filter_map(|i| i.ok())
            .collect::<Vec<_>>();
        Ok(items)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const DB_URL: &str = "./test.db3";

    #[test]
    fn init_database() {
        std::fs::remove_file(DB_URL).unwrap();

        let store = Store::open(DB_URL);
        assert!(store.is_ok());
        let store = store.unwrap();
        assert!(store.new_item().is_ok());
        let items = store.get_items();
        assert!(items.is_ok());
        let items = items.unwrap();
        assert_eq!(1, items.len());
    }
}

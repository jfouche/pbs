use std::{
    collections::HashMap,
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use crate::{
    database::{Database, ItemMaturity, ItemType},
    Error, Item, Result,
};

pub fn simple_8digits_pn_provider(db: &mut Database) -> Result<String> {
    const KEY: &str = "simple_pn_provider";
    let last_pn = db.read_config(KEY)?.parse::<usize>().unwrap_or(0);
    let new_pn = format!("{:08}", last_pn + 1);
    db.write_config(KEY, &new_pn)?;
    Ok(new_pn)
}

pub struct Store {
    db: RwLock<Database>,
}

unsafe impl Sync for Store {}

impl Store {
    /// Open the store
    pub fn open(url: &str) -> Result<Self> {
        let db = Database::open(url)?;
        Ok(Store {
            db: RwLock::new(db),
        })
    }

    fn db_read(&self) -> Result<RwLockReadGuard<'_, Database>> {
        let db = self.db.read().map_err(|_| Error::PoisonousDatabaseLock)?;
        Ok(db)
    }

    fn db_write(&mut self) -> Result<RwLockWriteGuard<'_, Database>> {
        let db = self.db.write().map_err(|_| Error::PoisonousDatabaseLock)?;
        Ok(db)
    }

    /// Get a config value from the database
    pub fn read_config(&self, key: &str) -> Result<String> {
        self.db_read()?.read_config(key)
    }

    /// Set a config value in the database
    pub fn write_config(&mut self, key: &str, value: &str) -> Result<()> {
        self.db_write()?.write_config(key, value)
    }

    /// Create a new item, allocating a new PN
    pub fn create_item(&mut self, name: &str) -> Result<Item> {
        let mut db = self.db_write()?;
        let pn = simple_8digits_pn_provider(&mut db)?;
        db.insert_item(&pn, name, ItemType::Internal)
    }

    // Add a exinsting item (e.g. existing PN) to the store
    pub fn import_item(&mut self, pn: &str, name: &str) -> Result<Item> {
        self.db_write()?.insert_item(pn, name, ItemType::External)
    }

    /// Save the item
    pub fn save_item(&mut self, item: Item) -> Result<()> {
        self.db
            .write()
            .map_err(|_| Error::PoisonousDatabaseLock)?
            .update_item(item)
    }

    /// Get all items
    pub fn items(&self) -> Result<Vec<Item>> {
        self.db
            .read()
            .map_err(|_| Error::PoisonousDatabaseLock)?
            .items()
    }

    /// Add a child to an item
    pub fn add_child(&mut self, parent_id: i64, child_id: i64, quantity: usize) -> Result<()> {
        let mut db = self.db.write().map_err(|_| Error::PoisonousDatabaseLock)?;
        db.add_child(parent_id, child_id, quantity)
    }

    /// Get all items children
    pub fn children(&self, id: i64) -> Result<Vec<(Item, usize)>> {
        self.db_read()?.children(id)
    }

    /// Get all parent items using the given item
    pub fn where_used(&self, id: i64) -> Result<Vec<Item>> {
        self.db_read()?.where_used(id)
    }

    /// Get all items and quantity that compose the given item
    pub fn stock(&self, id: i64) -> Result<HashMap<Item, usize>> {
        let mut stock = HashMap::new();
        for (child, quantity) in self.children(id)? {
            stock.extend(self.stock(child.id())?);
            *stock.entry(child).or_insert(0) += quantity;
        }
        Ok(stock)
    }

    /// Search for items matching pattern (pn or name)
    pub fn search_items(&self, pattern: &str) -> Result<Vec<Item>> {
        self.db_read()?.search(pattern)
    }

    /// Get an item by it's id
    pub fn item(&self, id: i64) -> Result<Item> {
        self.db_read()?.item_by_id(id)
    }

    /// Release an "in progress" Item
    pub fn release(&mut self, id: i64) -> Result<Item> {
        let item = self.db_read()?.item_by_id(id)?;
        if item.itype() != ItemType::Internal || item.maturity() != ItemMaturity::InProgress {
            Err(Error::CantReleaseItem)
        } else if self.can_release(id)? {
            let item = self.db_write()?.release(id)?;
            Ok(item)
        } else {
            Err(Error::CantReleaseItem)
        }
    }

    /// Return true if all children are [ItemMaturity::Released]
    fn can_release(&self, id: i64) -> Result<bool> {
        for child in self.db_read()?.children(id)? {
            if child.0.maturity() != ItemMaturity::Released {
                return Ok(false);
            }
            if !self.can_release(child.0.id())? {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

#[cfg(test)]
mod test {
    use crate::{Error, Store};

    #[test]
    fn release() {
        let mut store = Store::open(":memory:").expect("can(t open store");
        let parent = store.create_item("PARENT").unwrap();
        let item1 = store.create_item("ITEM1").unwrap();
        let item2 = store.create_item("ITEM2").unwrap();
        let cots1 = store.import_item("EXT.001", "COTS1").unwrap();
        let cots2 = store.import_item("EXT.002", "COTS2").unwrap();

        store.add_child(item1.id(), cots1.id(), 1).unwrap();
        store.add_child(item2.id(), cots1.id(), 1).unwrap();
        store.add_child(item2.id(), cots2.id(), 2).unwrap();
        store.add_child(parent.id(), item1.id(), 1).unwrap();
        store.add_child(parent.id(), item2.id(), 1).unwrap();

        assert_eq!(Ok(false), store.can_release(parent.id()));

        assert_eq!(
            Some(Error::CantReleaseItem),
            store.release(parent.id()).err()
        );

        assert!(store.release(item1.id()).is_ok());

        assert_eq!(
            Some(Error::CantReleaseItem),
            store.release(parent.id()).err()
        );

        assert!(store.release(item2.id()).is_ok());
        assert!(store.release(parent.id()).is_ok());
    }
}

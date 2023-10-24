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

    /// Shortcut to return a [Database] reader
    fn db_read(&self) -> Result<RwLockReadGuard<'_, Database>> {
        let db = self.db.read().map_err(|_| Error::PoisonousDatabaseLock)?;
        Ok(db)
    }

    /// Shortcut to return a [Database] writer
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

    /// Create a new [ItemType::Make] [Item], allocating a new PN
    pub fn make_item(&mut self, name: &str) -> Result<Item> {
        let mut db = self.db_write()?;
        let pn = simple_8digits_pn_provider(&mut db)?;
        db.insert_item(&pn, name, ItemType::Make)
    }

    /// Create a new [ItemType::Buy] [Item]
    pub fn buy_item(&mut self, pn: &str, name: &str) -> Result<Item> {
        self.db_write()?.insert_item(pn, name, ItemType::Buy)
    }

    /// Save the item
    pub fn save_item(&mut self, item: Item) -> Result<()> {
        self.db_write()?.update_item(item)
    }

    /// Get all items
    pub fn items(&self) -> Result<Vec<Item>> {
        self.db_read()?.items()
    }

    /// Add a child to an item
    pub fn add_child(&mut self, parent_id: i64, child_id: i64, quantity: usize) -> Result<()> {
        self.db_write()?.add_child(parent_id, child_id, quantity)
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
        self.db_read()?.item(id)
    }

    /// Release an "in progress" Item
    pub fn release(&mut self, id: i64) -> Result<Item> {
        let item = self.db_read()?.item(id)?;
        if item.itype() != ItemType::Make || item.maturity() != ItemMaturity::InProgress {
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

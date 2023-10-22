use std::{collections::HashMap, sync::RwLock};

use crate::{
    database::{Database, ItemType},
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

    /// Get a config value from the database
    pub fn read_config(&self, key: &str) -> Result<String> {
        self.db
            .read()
            .map_err(|_| Error::PoisonousDatabaseLock)?
            .read_config(key)
    }

    /// Set a config value in the database
    pub fn write_config(&mut self, key: &str, value: &str) -> Result<()> {
        self.db
            .write()
            .map_err(|_| Error::PoisonousDatabaseLock)?
            .write_config(key, value)
    }

    /// Create a new item, allocating a new PN
    pub fn create_item(&mut self, name: &str) -> Result<Item> {
        let mut db = self.db.write().map_err(|_| Error::PoisonousDatabaseLock)?;
        let pn = simple_8digits_pn_provider(&mut db)?;
        db.insert_item(&pn, name, ItemType::Internal)
    }

    // Add a exinsting item (e.g. existing PN) to the store
    pub fn import_item(&mut self, pn: &str, name: &str) -> Result<Item> {
        self.db
            .write()
            .map_err(|_| Error::PoisonousDatabaseLock)?
            .insert_item(pn, name, ItemType::External)
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
        let db = self.db.read().map_err(|_| Error::PoisonousDatabaseLock)?;
        db.children(id)
    }

    /// Get all parent items using the given item
    pub fn where_used(&self, id: i64) -> Result<Vec<Item>> {
        let db = self.db.read().map_err(|_| Error::PoisonousDatabaseLock)?;
        db.where_used(id)
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
        self.db
            .read()
            .map_err(|_| Error::PoisonousDatabaseLock)?
            .search(pattern)
    }

    /// Get an item by it's id
    pub fn item(&self, id: usize) -> Result<Item> {
        self.db
            .read()
            .map_err(|_| Error::PoisonousDatabaseLock)?
            .item_by_id(id)
    }

    /// Add a child to an item
    pub fn add_child_by_id(
        &mut self,
        parent_id: i64,
        child_id: i64,
        quantity: usize,
    ) -> Result<()> {
        let mut db = self.db.write().map_err(|_| Error::PoisonousDatabaseLock)?;
        db.add_child(parent_id, child_id, quantity)
    }
}

use std::{collections::HashMap, sync::RwLock};

use crate::{database::Database, Error, Item, Result};

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
    pub fn get_config(&self, key: &str) -> Result<String> {
        self.db
            .read()
            .map_err(|_| Error::PoisonousDatabaseLock)?
            .read_config(key)
    }

    /// Set a config value in the database
    pub fn set_config(&mut self, key: &str, value: &str) -> Result<()> {
        self.db
            .write()
            .map_err(|_| Error::PoisonousDatabaseLock)?
            .write_config(key, value)
    }

    /// Create a new item, allocating a new PN
    pub fn create_item(&mut self, name: &str) -> Result<Item> {
        let mut db = self.db.write().map_err(|_| Error::PoisonousDatabaseLock)?;
        let pn = simple_8digits_pn_provider(&mut db)?;
        db.insert_item(&pn, name)
    }

    // Add a exinsting item (e.g. existing PN) to the store
    pub fn import_item(&mut self, pn: &str, name: &str) -> Result<Item> {
        self.db
            .write()
            .map_err(|_| Error::PoisonousDatabaseLock)?
            .insert_item(pn, name)
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
    #[deprecated]
    pub fn add_child(&mut self, parent_pn: &str, child_pn: &str, quantity: usize) -> Result<()> {
        let mut db = self.db.write().map_err(|_| Error::PoisonousDatabaseLock)?;
        let parent_item = db.get_item_by_pn(parent_pn)?;
        let child_item = db.get_item_by_pn(child_pn)?;
        db.add_child(parent_item.id(), child_item.id(), quantity)
    }

    /// Get all items children
    #[deprecated]
    pub fn get_children(&self, pn: &str) -> Result<Vec<(Item, usize)>> {
        let db = self.db.read().map_err(|_| Error::PoisonousDatabaseLock)?;
        let item = db.get_item_by_pn(pn)?;
        db.children(&item)
    }

    /// Get all parent items using the given item
    pub fn where_used(&self, pn: &str) -> Result<Vec<Item>> {
        let db = self.db.read().map_err(|_| Error::PoisonousDatabaseLock)?;
        let item = db.get_item_by_pn(pn)?;
        db.where_used(&item)
    }

    /// Get all items and quantity that compose the given item
    pub fn get_stock(&self, pn: &str) -> Result<HashMap<Item, usize>> {
        let mut stock = HashMap::new();
        for (child, quantity) in self.get_children(pn)? {
            stock.extend(self.get_stock(child.pn())?);
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

    /// Get all items children
    pub fn children_by_id(&self, id: usize) -> Result<Vec<(Item, usize)>> {
        self.db
            .read()
            .map_err(|_| Error::PoisonousDatabaseLock)?
            .children_by_parent_id(id)
    }

    /// Get an item by it's id
    pub fn item_by_id(&self, id: usize) -> Result<Item> {
        self.db
            .read()
            .map_err(|_| Error::PoisonousDatabaseLock)?
            .item_by_id(id)
    }

    /// Add a child to an item
    pub fn add_child_by_id(
        &mut self,
        parent_id: usize,
        child_id: usize,
        quantity: usize,
    ) -> Result<()> {
        let mut db = self.db.write().map_err(|_| Error::PoisonousDatabaseLock)?;
        db.add_child(parent_id, child_id, quantity)
    }
}

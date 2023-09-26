use std::collections::HashMap;

use crate::{database::Database, Item, Result};

pub fn simple_8digits_pn_provider(db: &mut Database) -> Result<String> {
    const KEY: &str = "simple_pn_provider";
    let last_pn = db.get_config(KEY)?.parse::<usize>().unwrap_or(0);
    let new_pn = format!("{:08}", last_pn + 1);
    db.set_config(KEY, &new_pn)?;
    Ok(new_pn)
}

pub struct Store {
    db: Database,
}

impl Store {
    /// Open the store
    pub fn open(url: &str) -> Result<Self> {
        let db = Database::open(url)?;
        Ok(Store { db })
    }

    /// Get a config value from the database
    pub fn get_config(&self, key: &str) -> Result<String> {
        self.db.get_config(key)
    }

    /// Set a config value in the database
    pub fn set_config(&mut self, key: &str, value: &str) -> Result<()> {
        self.db.set_config(key, value)
    }

    /// Create a new item, allocating a new PN
    pub fn create(&mut self, name: &str) -> Result<Item> {
        let pn = simple_8digits_pn_provider(&mut self.db)?;
        self.db.insert_item(&pn, name)
    }

    // Add a new item to the store
    pub fn new_item(&mut self, pn: &str, name: &str) -> Result<Item> {
        self.db.insert_item(pn, name)
    }

    /// Save the item
    pub fn save_item(&mut self, item: Item) -> Result<()> {
        self.db.update_item(item)
    }

    /// Get all items
    pub fn get_items(&self) -> Result<Vec<Item>> {
        self.db.get_items()
    }

    /// Add a child to an item
    pub fn add_child(&mut self, parent_pn: &str, child_pn: &str, quantity: usize) -> Result<()> {
        let parent_item = self.db.get_item_by_pn(parent_pn)?;
        let child_item = self.db.get_item_by_pn(child_pn)?;
        self.db.add_child(&parent_item, &child_item, quantity)
    }

    /// Get all items children
    pub fn get_children(&self, pn: &str) -> Result<Vec<(Item, usize)>> {
        let item = self.db.get_item_by_pn(pn)?;
        self.db.get_children(&item)
    }

    /// Get all parent items using the given item
    pub fn where_used(&self, pn: &str) -> Result<Vec<Item>> {
        let item = self.db.get_item_by_pn(pn)?;
        self.db.where_used(&item)
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
        self.db.search(pattern)
    }
}

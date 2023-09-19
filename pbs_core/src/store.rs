use crate::{Database, Item, Result};

pub struct Store {
    db: Database,
}

impl Store {
    /// Open the store
    pub fn open(url: &str) -> Result<Self> {
        let db = Database::open(url)?;
        Ok(Store { db })
    }

    // Add a new item to the store
    pub fn new_item(&self, pn: &str, name: &str) -> Result<Item> {
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

    // Get all item children
    pub fn get_children(&self, pn: &str) -> Result<Vec<(Item, usize)>> {
        let item = self.db.get_item_by_pn(pn)?;
        self.db.get_children(&item)
    }
}

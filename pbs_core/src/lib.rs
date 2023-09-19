mod database;

pub use database::*;

// trait ErrConverter<T> {
//     fn if_err(err: Error) -> Result<T>;
// }

// impl<T> ErrConverter<T> for rusqlite::Result<T> {
//     fn if_err(err: Error) -> Result<T> {
//         Result::Err(err)
//     }
// }

pub struct Store {
    db: Database,
}

impl Store {
    /// Open the store
    pub fn open(url: &str) -> database::Result<Self> {
        let db = Database::open(url)?;
        Ok(Store { db })
    }

    // Add a new item to the store
    pub fn new_item(&self, pn: &str, name: &str) -> database::Result<Item> {
        self.db.insert_item(pn, name)
    }

    /// Save the item
    pub fn save_item(&mut self, item: Item) -> database::Result<()> {
        self.db.update_item(item)
    }

    pub fn get_items(&self) -> database::Result<Vec<Item>> {
        self.db.get_items()
    }

    pub fn add_child(
        &mut self,
        parent_pn: &str,
        child_pn: &str,
        quantity: usize,
    ) -> database::Result<()> {
        let parent_item = self.db.get_item_by_pn(parent_pn)?;
        let child_item = self.db.get_item_by_pn(child_pn)?;
        self.db.add_child(&parent_item, &child_item, quantity)
    }
}

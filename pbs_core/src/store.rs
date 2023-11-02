use std::{
    collections::HashMap,
    sync::{RwLock, RwLockReadGuard},
};

use crate::{
    database::{Children, Database, ItemMaturity, Strategy},
    Error, Item, Result,
};

pub fn simple_8digits_pn_provider(db: &Database) -> Result<String> {
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
    fn db(&self) -> Result<RwLockReadGuard<'_, Database>> {
        let db = self.db.read().map_err(|_| Error::PoisonousDatabaseLock)?;
        Ok(db)
    }

    /// Get a config value from the database
    pub fn read_config(&self, key: &str) -> Result<String> {
        self.db()?.read_config(key)
    }

    /// Set a config value in the database
    pub fn write_config(&self, key: &str, value: &str) -> Result<()> {
        self.db()?.write_config(key, value)
    }

    /// Create a new [ItemType::Make] [Item], allocating a new PN
    pub fn make_item(&self, name: &str) -> Result<Item> {
        let db = self.db()?;
        let pn = simple_8digits_pn_provider(&db)?;
        db.insert_item(&pn, name, Strategy::Make)
    }

    /// Create a new [ItemType::Buy] [Item]
    pub fn buy_item(&self, pn: &str, name: &str) -> Result<Item> {
        self.db()?.insert_item(pn, name, Strategy::Buy)
    }

    /// Get all items
    pub fn items(&self) -> Result<Vec<Item>> {
        self.db()?.items()
    }

    /// Add a child to an item
    ///
    /// An item can only be a child of an [Strategy::Make] and [ItemMaturity::InProgress] item
    pub fn add_child(&self, parent_id: i64, child_id: i64, quantity: usize) -> Result<()> {
        let db = self.db()?;
        let parent = db.item(parent_id)?;
        if parent.strategy() != Strategy::Make || parent.maturity() != ItemMaturity::InProgress {
            Err(Error::CantAddChild)
        } else {
            db.add_child(parent_id, child_id, quantity)
        }
    }

    pub fn remove_child(&self, parent_id: i64, child_id: i64) -> Result<()> {
        let db = self.db()?;
        let parent = db.item(parent_id)?;
        if parent.strategy() != Strategy::Make || parent.maturity() != ItemMaturity::InProgress {
            Err(Error::CantRemoveChild)
        } else {
            db.delete_child(parent_id, child_id)
        }
    }

    /// Get all items children
    pub fn children(&self, id: i64) -> Result<Children> {
        self.db()?.children(id)
    }

    /// Get all parent items using the given item
    pub fn where_used(&self, id: i64) -> Result<Vec<Item>> {
        self.db()?.where_used(id)
    }

    /// Get all items and quantity that compose the given item
    pub fn stock(&self, id: i64) -> Result<HashMap<i64, usize>> {
        let mut stock = HashMap::new();
        for child in &self.children(id)? {
            stock.extend(self.stock(child.id())?);
            *stock.entry(child.id()).or_insert(0) += child.quantity();
        }
        Ok(stock)
    }

    /// Search for items matching pattern (pn or name)
    pub fn search_items(&self, pattern: &str) -> Result<Vec<Item>> {
        self.db()?.search(pattern)
    }

    /// Get an item by it's id
    pub fn item(&self, id: i64) -> Result<Item> {
        self.db()?.item(id)
    }

    /// Release an "in progress" Item
    pub fn release(&self, id: i64) -> Result<Item> {
        let item = self.db()?.item(id)?;
        if item.strategy() != Strategy::Make || item.maturity() != ItemMaturity::InProgress {
            Err(Error::CantReleaseItem)
        } else if self.can_release(id)? {
            let item = self.db()?.release(id)?;
            Ok(item)
        } else {
            Err(Error::CantReleaseItem)
        }
    }

    /// Return true if all children are [ItemMaturity::Released]
    ///
    /// This function is recursive
    fn can_release(&self, id: i64) -> Result<bool> {
        for child in &self.db()?.children(id)? {
            if child.maturity() != ItemMaturity::Released {
                return Ok(false);
            }
            if !self.can_release(child.id())? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// Make a [Strategy::Buy] item obsolete
    ///
    /// All parent item will switch to [ItemMaturity::Obsolete]
    pub fn make_obsolete(&self, id: i64) -> Result<Item> {
        let db = self.db()?;
        if db.item(id)?.strategy() != Strategy::Buy {
            Err(Error::CantMakeObsolete)
        } else {
            self.make_where_used_obsolete(id)?;
            db.item(id)
        }
    }

    /// Recursively mark item and its parents [ItemMaturity::Obsolete] if it's not [ItemMaturity::InProgress]
    fn make_where_used_obsolete(&self, id: i64) -> Result<()> {
        let db = self.db()?;
        if db.item(id)?.maturity() != ItemMaturity::InProgress {
            // mark item as obsolete...
            db.make_obsolete(id)?;
            // ..  as well as its parents
            for parent in db.where_used(id)? {
                assert_eq!(Strategy::Make, parent.strategy());
                self.make_where_used_obsolete(parent.id())?;
            }
        }
        Ok(())
    }
}

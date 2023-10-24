mod database;
mod store;
#[cfg(test)]
mod tests;

pub use database::{Database, Item, ItemMaturity, ItemType};
pub use store::Store;

#[derive(Debug, PartialEq)]
pub enum Error {
    DatabaseErr(rusqlite::Error),
    PoisonousDatabaseLock,
    CantReleaseItem,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

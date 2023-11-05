mod database;
mod store;
#[cfg(test)]
mod tests;

pub use database::{Database, Item, ItemMaturity, Strategy};
pub use store::{Child, Children, Stock, Store};

#[derive(Debug, PartialEq)]
pub enum Error {
    DatabaseErr(rusqlite::Error),
    PoisonousDatabaseLock,
    CantReleaseItem,
    CantAddChild,
    CantRemoveChild,
    CantMakeObsolete,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

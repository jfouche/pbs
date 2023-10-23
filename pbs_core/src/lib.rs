mod database;
mod store;

pub use database::{Database, Item};
pub use store::Store;

#[derive(Debug, PartialEq)]
pub enum Error {
    DatabaseErr(rusqlite::Error),
    PoisonousDatabaseLock,
    CantReleaseItem,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

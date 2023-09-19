mod database;
mod store;

pub use database::{Database, Item};
pub use store::Store;

#[derive(Debug)]
pub enum Error {
    DatabaseErr(rusqlite::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

// trait ErrConverter<T> {
//     fn if_err(err: Error) -> Result<T>;
// }

// impl<T> ErrConverter<T> for rusqlite::Result<T> {
//     fn if_err(err: Error) -> Result<T> {
//         Result::Err(err)
//     }
// }

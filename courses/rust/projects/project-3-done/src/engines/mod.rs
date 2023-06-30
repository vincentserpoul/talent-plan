pub mod kvs;

pub use kvs::KvStore;

use crate::errors::Result;

/// Engine trait, which defines the interface of a key/value store.
pub trait KvsEngine {
    /// Set the value of a string key to a string.
    fn set(&mut self, key: String, value: String) -> Result<()>;
    /// Get the string value of a given string key.
    fn get(&mut self, key: String) -> Result<Option<String>>;
    /// Remove a given string key.
    fn remove(&mut self, key: String) -> Result<()>;
}

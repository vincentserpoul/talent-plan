pub mod kvs;
pub mod sled;

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

use serde::{Deserialize, Serialize};

/// Commands enum, which defines the commands that can be sent to the server.
#[derive(Debug, Serialize, Deserialize)]
pub enum KvCommand {
    /// Get the string value of a given string key.
    Get(String),
    /// Set the value of a string key to a string.
    Set(String, String),
    /// Remove a given string key.
    Remove(String),
}

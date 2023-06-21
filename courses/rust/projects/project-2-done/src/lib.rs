#![deny(missing_docs)]
//! A simple key/value store.

pub use errors::Result;
pub use kv::KvStore;

mod errors;
mod kv;

#![deny(missing_docs)]
//! A simple key/value store.

pub use engines::KvStore;
pub use engines::KvsEngine;
pub use errors::Result;

mod client;
mod engines;
mod errors;
mod server;

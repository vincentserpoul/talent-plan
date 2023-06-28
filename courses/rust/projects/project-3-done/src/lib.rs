#![deny(missing_docs)]
//! A simple key/value store.

pub use errors::Result;
pub use store::KvStore;

mod client;
mod engine;
mod errors;
mod server;
mod store;

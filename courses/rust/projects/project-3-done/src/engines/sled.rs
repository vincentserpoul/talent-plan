use crate::errors::{KvsError, Result};
use crate::KvsEngine;
use sled::{Db, Tree};

impl KvsEngine for Db {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        let tree: &Tree = self;
        tree.insert(key, value.into_bytes())?;

        Ok(())
    }

    fn get(&mut self, key: String) -> Result<Option<String>> {
        let tree: &Tree = self;
        let res = tree.get(key)?;
        if res.is_none() {
            return Ok(None);
        };

        Ok(res
            .map(|i_vec| AsRef::<[u8]>::as_ref(&i_vec).to_vec())
            .map(String::from_utf8)
            .transpose()?)
    }

    fn remove(&mut self, key: String) -> Result<()> {
        let tree: &Tree = self;
        let res = tree.get(&key)?;
        if res.is_none() {
            return Err(KvsError::KeyNotFound);
        };

        tree.remove(key)?;
        tree.flush()?;
        Ok(())
    }
}

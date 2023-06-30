use crate::errors::{KvsError, Result};
use serde_json::json;
use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};

/// The `KvStore` stores string key/value pairs.
pub struct KvStore {
    map: HashMap<String, String>,
    path: PathBuf,
}

const LOG_FILE: &str = "log";

impl KvStore {
    /// Open a `KvStore` from a given path.
    pub fn open(_path: &Path) -> Result<KvStore> {
        std::fs::create_dir_all(_path)?;

        let mut kv_store = KvStore {
            map: HashMap::new(),
            path: _path.to_owned(),
        };

        kv_store.read_log()?;

        Ok(kv_store)
    }

    /// Read the log from file and apply to the hashmap
    /// This is called on open
    fn read_log(&mut self) -> Result<()> {
        let mut path = self.path.clone();
        path.push(LOG_FILE);

        // if file can be opened, read it
        // otherwise exit with no error
        let file = match std::fs::OpenOptions::new().read(true).open(path) {
            Ok(file) => file,
            Err(_) => return Ok(()),
        };

        let reader = std::io::BufReader::new(file);

        for line in reader.lines() {
            let command: KvCommand = serde_json::from_str(&line?)?;
            match command {
                KvCommand::Set(key, value) => {
                    self.map.insert(key, value);
                }
                KvCommand::Remove(key) => {
                    self.map.remove(&key);
                }
                KvCommand::Get(_) => (),
            }
        }

        Ok(())
    }

    /// Set the value of a string key to a string.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.map.insert(key, value);

        Ok(())
    }

    /// Get the string value of a given string key.
    pub fn get(&self, key: String) -> Result<Option<String>> {
        match self.map.get(&key) {
            Some(value) => Ok(Some(value.to_owned())),
            None => Ok(None),
        }
    }

    /// Remove a given string key.
    pub fn remove(&mut self, key: String) -> Result<()> {
        // check that the key exists first
        if !self.map.contains_key(&key) {
            return Err(KvsError::KeyNotFound);
        }

        self.map.remove(&key);

        Ok(())
    }
}

impl Drop for KvStore {
    /// rewrite the log with the corresponding map
    fn drop(&mut self) {
        // delete the log file
        let mut path = self.path.clone();
        path.push(LOG_FILE);

        // create a new log file
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .unwrap();

        // iterate through the map and write insert commands for the existing keys
        // to the log file
        let mut writer = std::io::BufWriter::new(file);
        for (key, value) in &self.map {
            let command = KvCommand::Set(key.to_owned(), value.to_owned());
            writeln!(writer, "{}", json!(command)).unwrap();
        }

        // close the file
        writer.flush().unwrap();
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum KvCommand {
    Get(String),
    Set(String, String),
    Remove(String),
}

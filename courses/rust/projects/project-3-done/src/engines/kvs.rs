use crate::errors::{KvsError, Result};
use crate::KvsEngine;
use serde_json::json;
use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};

use super::KvCommand;

/// The `KvStore` stores string key/value pairs.
pub struct KvStore {
    map: HashMap<String, String>,
    last_compacted: u64,
    path: PathBuf,
}

const LOG_FILE: &str = "log";
const COMPACT_THRESHOLD: u64 = 100;

impl KvStore {
    /// Open a `KvStore` from a given path.
    pub fn open(path: &Path) -> Result<KvStore> {
        std::fs::create_dir_all(path)?;
        let mut path = path.to_owned();
        path.push(LOG_FILE);

        let mut kv_store = KvStore {
            map: HashMap::new(),
            last_compacted: 0,
            path,
        };

        kv_store.read_log()?;

        Ok(kv_store)
    }

    /// Read the log from file and apply to the hashmap
    /// This is called on open
    fn read_log(&mut self) -> Result<()> {
        // if file can be opened, read it
        // otherwise exit with no error
        let file = match std::fs::OpenOptions::new().read(true).open(&self.path) {
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

    fn compact(&mut self) {
        // create a new log file
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)
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

        self.last_compacted = 0;
    }
}

impl KvsEngine for KvStore {
    /// Set the value of a string key to a string.
    fn set(&mut self, key: String, value: String) -> Result<()> {
        self.map.insert(key.clone(), value.clone());

        let file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&self.path)
            .unwrap();

        // write the command into the log file
        let mut writer = std::io::BufWriter::new(file);
        writeln!(writer, "{}", json!(KvCommand::Set(key, value)))?;
        writer.flush()?;

        self.last_compacted += 1;

        if self.last_compacted > COMPACT_THRESHOLD {
            self.compact();
        }

        Ok(())
    }

    /// Get the string value of a given string key.
    fn get(&mut self, key: String) -> Result<Option<String>> {
        match self.map.get(&key) {
            Some(value) => Ok(Some(value.to_owned())),
            None => Ok(None),
        }
    }

    /// Remove a given string key.
    fn remove(&mut self, key: String) -> Result<()> {
        // check that the key exists first
        if !self.map.contains_key(&key) {
            return Err(KvsError::KeyNotFound);
        }

        self.map.remove(&key);

        let file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&self.path)
            .unwrap();

        // write the command into the log file
        let mut writer = std::io::BufWriter::new(file);
        writeln!(writer, "{}", json!(KvCommand::Remove(key)))?;
        writer.flush()?;

        self.last_compacted += 1;

        if self.last_compacted > COMPACT_THRESHOLD {
            self.compact();
        }

        Ok(())
    }
}

impl Drop for KvStore {
    /// rewrite the log with the corresponding map
    fn drop(&mut self) {
        self.compact();
    }
}

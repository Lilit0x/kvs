#![deny(missing_docs)]
//! A simple in-memory key/value store that maps strings to strings
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    result::Result as StdResult,
};

use failure::Fail;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// KvsErrors
#[derive(Fail, Debug)]
pub enum KvsError {
    /// IO Error
    #[fail(display = "{}", _0)]
    Io(#[cause] std::io::Error),
    /// Serde Error
    #[fail(display = "{}", _0)]
    Json(#[cause] serde_json::Error),
}

impl From<std::io::Error> for KvsError {
    fn from(value: std::io::Error) -> Self {
        KvsError::Io(value)
    }
}

impl From<serde_json::Error> for KvsError {
    fn from(value: serde_json::Error) -> Self {
        KvsError::Json(value)
    }
}

/// KVS Result type
pub type Result<T> = StdResult<T, KvsError>;

/// The main store
pub struct KvStore {
    store: HashMap<String, u64>,
    log_file: PathBuf,
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}

impl KvStore {
    /// Initialize a new insyance of the store
    /// ```rust
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// ```
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            ..Default::default()
        }
    }

    /// Create a KvStore from a file
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let store = Self {
            log_file: path.as_ref().to_path_buf(),
            store: HashMap::new(),
        };

        Ok(store)
    }

    /// Set the value of a string key to a string
    /// ```rust
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set("key1".to_owned(), "value1".to_owned());
    /// assert_eq!(store.get("key1".to_owned()), Some("value1".to_owned()));
    /// ```
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd = Command::Set {
            key: key.clone(),
            value,
        };
        let val = serde_json::to_string(&cmd)?;

        let mut log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_file)?;
        // get the beginning of where the write will occur.
        // so, if we want to get an element from the index, we just take everything from that offset
        // up to the next "new line", since that's the delimeter
        let offset = log_file.seek(SeekFrom::Current(0))?;
        let writer = BufWriter::new(&log_file);

        serde_json::to_writer(writer, &val)?;

        // the delimter should be a new line
        writeln!(log_file)?;
        log_file.flush()?;

        self.store.insert(key, offset);
        Ok(())
    }

    /// Get the string value of a given string key
    /// ```rust
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set("key1".to_owned(), "value1".to_owned());
    /// store.set("key2".to_owned(), "value2".to_owned());
    /// assert_eq!(store.get("key1".to_owned()), Some("value1".to_owned()));
    /// assert_eq!(store.get("key2".to_owned()), Some("value2".to_owned()));
    /// ```
    pub fn get(&self, key: String) -> Result<Option<String>> {
        let log_file = File::open(&self.log_file)?;
        let lines = BufReader::new(log_file).lines();
        
        // so, I need to get the file offset of each line too (that is, where the line begins)
        // depending on the command, if it is set, i insert into the map, it is remove, I remove from the map
        // I don't know what to do with Get yet
        for line in lines {
            let line = line?;
            let command: Command = serde_json::from_str(&line)?;
            println!("{:#?}", command);
            self.store.insert(k, v)
        }

        let _key = self.store.get(&key).cloned();
        todo!()
    }

    /// Remove a given key
    /// ```rust
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    ///  store.set("key1".to_owned(), "value1".to_owned());
    /// store.remove("key1".to_owned());
    /// assert_eq!(store.get("key1".to_owned()), None);
    /// ```
    pub fn remove(&mut self, key: String) -> Result<()> {
        if let Some(_) = self.store.remove(&key) {
            let val = serde_json::to_string(&Command::Rm { key })?;
            let writer = BufWriter::new(File::open(&self.log_file)?);
            serde_json::to_writer(writer, &val)?;
        };
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Set { key: String, value: String },
    Get { key: String },
    Rm { key: String },
}

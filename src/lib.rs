#![deny(missing_docs)]
//! A simple in-memory key/value store that maps strings to strings
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufWriter, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    result::Result as StdResult,
};

use failure::Fail;
use serde::{Deserialize, Serialize};

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
        let writer = BufWriter::new(&log_file);

        serde_json::to_writer(writer, &val)?;
        let offset = log_file.seek(SeekFrom::Current(0))?;
        log_file.flush()?;

        self.store.insert(key, offset);
        // TODO: the file offset is the end, I don't know how I'll read to get the contents of the file
        // do I need to delimit with a new line?
        println!("{:#?}", &self.store);
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

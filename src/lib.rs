#![deny(missing_docs)]
//! A simple in-memory key/value store that maps strings to strings
use std::{collections::HashMap, io, path::{Path, PathBuf}, result::Result as StdResult};

use failure::Fail;


/// KvsErrors 
#[derive(Fail, Debug)]
pub enum KvsError {
    /// IO Error
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),
}

/// KVS Result type
pub type Result<T> = StdResult<T, KvsError>;

/// The main store
pub struct KvStore {
    store: HashMap<String, String>,
    log_file: PathBuf
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
            log_file: Default::default()
        }
    }

    /// Create a KvStore from a file 
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        todo!()
    }

    /// Set the value of a string key to a string
    /// ```rust
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set("key1".to_owned(), "value1".to_owned());
    /// assert_eq!(store.get("key1".to_owned()), Some("value1".to_owned()));
    /// ```
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
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
    pub fn get(&self, key: String) -> Option<String> {
        self.store.get(&key).cloned()
    }

    /// Remove a given key
    /// ```rust
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    ///  store.set("key1".to_owned(), "value1".to_owned());
    /// store.remove("key1".to_owned());
    /// assert_eq!(store.get("key1".to_owned()), None);
    /// ```
    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
}

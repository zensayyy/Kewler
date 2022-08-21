use std::{collections::HashMap, fs::File, path::PathBuf};

use anyhow::{bail, Context, Result};
use rocket::serde::DeserializeOwned;
use serde::{Deserialize, Serialize};

use super::database::Database;

#[derive(Serialize, Deserialize)]
pub struct KvStore<V> {
    pub databases: HashMap<String, Database<V>>,
}

impl<V: DeserializeOwned> KvStore<V> {
    /// init a new KvStore
    fn new() -> KvStore<V> {
        KvStore {
            databases: HashMap::new(),
        }
    }

    /// load a KvStore from file on disk, integrity checks transitively by deserializing
    fn from_file(path: &PathBuf) -> Result<KvStore<V>> {
        // Open the file
        let file = File::open(path).context(format!(
            "Failed to open the File '{}'",
            path.to_str().unwrap_or("")
        ))?;

        let kv_store: KvStore<V> =
        // integrity check included
            bson::from_reader(file).context("Failed to deserialize FsStore")?;
        Ok(kv_store)
    }

    /// Load or init the KvStore from the given PathBuf
    pub fn load_or_init(kv_store_path: PathBuf) -> Result<KvStore<V>> {
        // Check if file exists
        if !kv_store_path.exists() {
            return Ok(KvStore::new());
        }

        // deserialize the KvStore from file
        let kv_store = match KvStore::<V>::from_file(&kv_store_path) {
            Err(_) => {
                // TODO: handle deserializing error e.g. init new?
                bail!(format!(
                    "Failed to deserialize '{}', might be corrupted...",
                    kv_store_path.to_str().unwrap_or("{}")
                ))
            }
            Ok(kv_store) => kv_store,
        };

        Ok(kv_store)
    }
}

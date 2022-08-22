use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    path::PathBuf,
};

use super::database::Database;
use anyhow::{bail, Context, Result};
use rocket::serde::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Serialize, Deserialize)]
pub struct KvStore<V: Serialize> {
    #[serde(skip_serializing, skip_deserializing)]
    pub kv_store_path: PathBuf,
    pub databases: HashMap<String, Database<V>>,
}

impl<V: DeserializeOwned + Serialize> KvStore<V> {
    /// init a new KvStore
    fn new(path: PathBuf) -> KvStore<V> {
        KvStore {
            kv_store_path: path,
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
            return Ok(KvStore::new(kv_store_path));
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
            Ok(kv_store) => {
                let kv_store = KvStore {
                    kv_store_path: kv_store_path,
                    databases: kv_store.databases,
                };
                kv_store
            }
        };

        Ok(kv_store)
    }

    /// Save the KvStore to given path
    pub fn save(&self) -> Result<()> {
        let kv_store_bytes = bson::to_vec(self).context("Failed to serialize the KvStore")?;
        let mut file = OpenOptions::new()
            .truncate(true)
            .write(true)
            .create(true)
            .open(&self.kv_store_path)
            .context(format!(
                "Failed to open File to save KvStore '{}'",
                self.kv_store_path.as_os_str().to_str().unwrap_or("")
            ))?;
        Ok(file.write_all(&kv_store_bytes)?)
    }

    /// adds a named database to the store, if not present
    /// however, does not persist the KvStore yet
    /// return `None` if the database already exists, otherwise returns Some(())
    pub fn add_database(&mut self, name: &str) -> Option<()> {
        if self.databases.contains_key(name) {
            return None;
        }
        self.databases.insert(name.to_string(), Database::new());
        Some(())
    }

    /// removes a named database
    /// however, does not persist the KvStore yet
    /// returns `None`if database did not exists, otherwise returns Some(())
    pub fn drop_database(&mut self, name: &str) -> Option<()> {
        if let Some(_) = self.databases.remove(name) {
            return Some(());
        }
        None
    }
}

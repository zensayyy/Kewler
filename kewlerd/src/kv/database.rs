use std::{collections::HashMap, hash::Hash};

use anyhow::Result;

/// Database that holds the data, identified by `name`
pub struct Database<V> {
    name: String,
    data: HashMap<String, V>
}

impl<V> Database<V> {
    // internal function
    fn new(name: &str) -> Database<V> {
        Database {
            name: name.to_string(),
            data: HashMap::new()
        }
    }

    pub fn load_or_init(name: &str) -> Result<Database<V>> {
        todo!()
    }

}
use std::{collections::HashMap, hash::Hash};

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Database that holds the data, identified by `name`
#[derive(Serialize, Deserialize)]
pub struct Database<V> {
    name: String,
    data: HashMap<String, V>,
}

impl<V> Database<V> {
    // internal function
    fn new(name: &str) -> Database<V> {
        Database {
            name: name.to_string(),
            data: HashMap::new(),
        }
    }
}

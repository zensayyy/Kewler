use std::{collections::HashMap, hash::Hash};

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Database that holds the data, identified by `name`
#[derive(Serialize, Deserialize)]
pub struct Database<V> {
    data: HashMap<String, V>,
}

impl<V> Database<V> {
    pub fn new() -> Database<V> {
        Database {
            data: HashMap::new(),
        }
    }
}

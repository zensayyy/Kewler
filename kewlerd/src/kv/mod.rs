use std::{fs::File, path::PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use self::store::KvStore;

pub mod database;
pub mod store;

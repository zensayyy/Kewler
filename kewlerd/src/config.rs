use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct DaemonConfig {   
    kv_store_path: PathBuf
}
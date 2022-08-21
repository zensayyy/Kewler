use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct DaemonConfig {
    pub kv_store_path: PathBuf,
}

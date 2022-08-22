use serde::{Deserialize, Serialize};

pub mod git;

/// implemented models, storable in KvStore
#[derive(Deserialize, Serialize)]
pub enum KewlModel {
    Github(git::GitObject),
}

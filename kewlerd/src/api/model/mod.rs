use serde::Deserialize;

pub mod git;

/// implemented models, storable in KvStore
#[derive(Deserialize)]
pub enum KewlModel {
    Github(git::GitObject),
}

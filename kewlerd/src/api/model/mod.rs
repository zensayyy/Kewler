pub mod git;

/// implemented models, storable in KvStore
pub enum KewlModel {
    Github(git::GitObject)
}
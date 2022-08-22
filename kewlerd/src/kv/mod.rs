use std::{fs::File, path::PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use self::store::KvStore;

pub mod database;
pub mod store;

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::api::model::{
        git::{GitObject, GitRepov1, Secret},
        KewlModel,
    };

    use super::store::KvStore;

    /*
        Case to cover for KvStore:
        1- not initialized yet: load_or_init()
        2- initialized with some data: load_or_init(), should also test save, add/drop database implicitly

    */

    const filename: &'static str = "/tmp/test_KvStore"; // TODO: alternative to be more platform independent

    // Data
    fn gimme_data() -> (PathBuf, KewlModel, KewlModel) {
        let kv_store_path: PathBuf = PathBuf::from(filename);
        let repo1: KewlModel = KewlModel::Github(GitObject::v1(GitRepov1 {
            url: "https://github.com/rust-lang/rust".to_owned(),
            secret: None,
            interval: 60,
        }));
        let repo2: KewlModel = KewlModel::Github(GitObject::v1(GitRepov1 {
            url: String::from("https://github.com/zensayyy/Kewler"),
            secret: Some(Secret {
                username: "username".to_owned(),
                password: "password".to_owned(),
            }),
            interval: 60,
        }));
        (kv_store_path, repo1, repo2)
    }

    fn flush() {
        std::fs::remove_file(filename);
    }

    fn init_data() {
        flush();
        let (kv_store_path, repo1, repo2) = gimme_data();
        let mut store = KvStore::<KewlModel>::load_or_init(kv_store_path.to_owned()).unwrap();
        store.add_database("github.com").unwrap();
        store.drop_database("github.com").unwrap();
        store.add_database("github.com").unwrap();
        store.save().unwrap();
    }

    // case 1
    #[test]
    #[serial_test::serial]
    fn load_or_init_empty() {
        let (kv_store_path, repo1, repo2) = gimme_data();
        let store = KvStore::<KewlModel>::load_or_init(kv_store_path.to_owned()).unwrap();
        assert!(store.kv_store_path == kv_store_path);
        assert!(store.databases.is_empty());
    }

    // case 2
    #[test]
    #[serial_test::serial]
    fn load_or_init_with_pre_data() {
        init_data();
        let (kv_store_path, repo1, repo2) = gimme_data();
        let store = KvStore::<KewlModel>::load_or_init(kv_store_path.to_owned()).unwrap();
        assert!(store.kv_store_path == kv_store_path);
        assert!(!store.databases.is_empty());
        assert!(store.databases.contains_key("github.com"));
        flush();
    }
}

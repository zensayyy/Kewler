#[macro_use]
extern crate rocket;

mod api;
mod config;
mod kv;

use std::{collections::HashMap, sync::Mutex};

use api::{controller, model::KewlModel};
use kv::{database::Database, store::KvStore};

#[launch]
fn rocket() -> _ {
    // TODO: Load config
    let conf: config::DaemonConfig = toml::from_str(
        r#"
        kv_store_path = "/tmp/kewler_kv_store"
    "#,
    )
    .unwrap();

    // init the KvStore

    let store = KvStore::<KewlModel>::load_or_init(conf.kv_store_path).unwrap();

    rocket::build().manage(store).mount(
        "/",
        routes![
            controller::git::add,
            controller::git::remove,
            controller::git::update
        ],
    )
}

#[macro_use]
extern crate rocket;

mod api;
mod kv;
mod config;

use std::{collections::HashMap, sync::Mutex};

use api::{controller, model::KewlModel};
use kv::{store::KvStore, database::Database};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(
            KvStore {
                database: vec![
                    Database::<KewlModel>::load_or_init("github").unwrap()
                ]    
            }
        )
        .mount(
            "/",
            routes![
                controller::git::add,
                controller::git::remove,
                controller::git::update
            ],
        )
}

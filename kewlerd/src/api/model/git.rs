use rocket::serde::{Deserialize, Serialize};

pub enum GitObject {
    v1(GitRepov1)
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(crate = "rocket::serde")]
/// Secret used for authentication at private repositories
pub struct Secret {
    username: String,
    password: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
/// documented in API.md
pub struct GitRepov1 {
    apiVersion: String,
    url: String,
    secret: Option<Secret>,
    interval: u32,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
/// documented in API.md
pub struct GitRepoUpdate {
    apiVersion: String,
    url: String,
    secret: Option<Secret>,
    interval: Option<u32>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
/// documented in API.md
pub struct GitRepoRm {
    apiVersion: String,
    url: String,
}

use super::Kewl;
use crate::api::model::git;
use rocket::serde::json::Json;
/// As descriped in API.md
#[post("/git/add", data = "<repo>")]
pub async fn add(repo: Json<git::GitRepov1>) -> Kewl {
    // TODO
    Kewl::Ok("Created".to_string())
}

#[post("/git/remove", data = "<rm>")]
pub async fn remove(rm: Json<git::GitRepoRm>) -> Kewl {
    // TODO
    Kewl::Ok("Removed".to_string())
}

#[post("/git/update", data = "<update>")]
pub async fn update(update: Json<git::GitRepoUpdate>) -> Kewl {
    // TODO
    Kewl::Ok("Updated".to_string())
}

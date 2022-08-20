pub mod git;

/// Used as response for api requests
#[derive(Responder)]
pub enum Kewl {
    #[response(status = 200, content_type = "text")]
    Ok(String),
    #[response(status = 401, content_type = "text")]
    Unauthorized(String),
    #[response(status = 500, content_type = "text")]
    Error(String)
}
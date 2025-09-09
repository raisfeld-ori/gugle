use gurt::{GurtResponse, Result};
use sqlite::Connection;

pub fn get(body: String) -> Result<GurtResponse> {
    Ok(GurtResponse::ok().with_string_body(""))
}

pub fn post(body: String) -> Result<GurtResponse> {
    Ok(GurtResponse::ok().with_string_body(String::from("User created")))
}

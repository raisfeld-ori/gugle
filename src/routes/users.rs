use gurt::{GurtResponse, Result};
use sqlite::Connection;

pub fn get(body: string) -> Result<GurtResponse> {
    Ok(GurtResponse::ok().with_string_body(""))
}

pub fn post(body: string) -> Result<GurtResponse> {
    Ok(GurtResponse::ok().with_string_body(String::from("User created")))
}

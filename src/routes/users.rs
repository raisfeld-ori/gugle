use gurt::{GurtResponse, Result};

pub fn get() -> Result<GurtResponse> {
    Ok(GurtResponse::ok().with_string_body(String::from("User list")))
}

pub fn post() -> Result<GurtResponse> {
    Ok(GurtResponse::ok().with_string_body(String::from("User created")))
}

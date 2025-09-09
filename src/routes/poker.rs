use gurt::{GurtResponse, Result};
use std::fs;
use std::path::Path;

pub fn get() -> Result<GurtResponse> {
    let html_path = Path::new("src/routes/frontend/poker.gurt");
    let html_content = fs::read_to_string(html_path);
    if html_content.is_err() {
        return Ok(
            GurtResponse::internal_server_error().with_string_body("500 Internal Server Error")
        );
    }
    Ok(GurtResponse::ok().with_string_body(&html_content.unwrap()))
}

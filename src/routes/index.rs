use gurt::{GurtResponse, Result};
use std::fs;
use std::path::Path;


pub fn index() -> Result<GurtResponse> {
    let html_path = Path::new("src/routes/frontend/index.html");
    let html_content = fs::read_to_string(html_path)
        .unwrap_or_else(|_| "<h1>Could not read index.html</h1>".to_string());
    Ok(GurtResponse::ok().with_string_body(&html_content))
}
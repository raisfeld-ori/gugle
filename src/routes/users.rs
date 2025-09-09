use gurt::{GurtResponse, Result, ServerContext};

pub fn get() -> Result<GurtResponse> {
    return Ok(GurtResponse::internal_server_error().with_string_body("500 Internal Server Error"));

}
pub fn post() -> Result<GurtResponse>{
    return Ok(GurtResponse::internal_server_error().with_string_body("500 Internal Server Error"));
}
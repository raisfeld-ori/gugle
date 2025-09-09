use gurt::{GurtResponse, Result};
use sqlite::Connection;

#[derive(Debug)]
struct User {
    name: String,
    password: String,
    currency: i32,
    create: bool
}

pub fn get(body: String) -> Result<GurtResponse> {
    let db = Connection::open("gambling.db").unwrap();
    let query = format!("SELECT * FROM users WHERE name = '{}'", body);
    Ok(GurtResponse::ok().with_string_body(""))
}

pub fn post(body: String) -> Result<GurtResponse> {
    let db = Connection::open("gambling.db").unwrap();
    let user_data: Vec<&str> = body.split(',').collect();
    let query = format!(
        "INSERT INTO users (name, password, currency) VALUES ('{}', '{}', '{}')",
        user_data[0], user_data[1], user_data[2]
    );
    db.execute(query).unwrap();
    Ok(GurtResponse::ok().with_string_body(String::from("User created")))
}

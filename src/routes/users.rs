use gurt::{GurtResponse, Result};
use serde::{Deserialize, Serialize};
use sqlite::Connection;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    name: String,
    password: String,
    currency: f64,
    create: bool
}

pub fn get(body: String) -> Result<GurtResponse> {
    let body = serde_json::from_str::<User>(&body);
    if (body.is_err()) {
        return Ok(GurtResponse::bad_request().with_string_body(String::from("Bad request")));
    }
    let body = body.unwrap();
    let db = Connection::open("gambling.db").unwrap();
    let query = format!("SELECT * FROM users WHERE name = '{}' AND password = '{}'", body.name, body.password);
    let mut statement = db.prepare(query).unwrap();
    let mut users: Vec<User> = Vec::new();
    while let sqlite::State::Row = statement.next().unwrap() {
        let user = User {
            name: statement.read::<String, _>("name").unwrap(),
            password: statement.read::<String, _>("password").unwrap(),
            currency: statement.read::<f64, _>("currency").unwrap(),
            create: false
        };
        users.push(user);
    }
    if users.len() == 0 {
        return Ok(GurtResponse::ok().with_string_body(String::from("User not found")));
    } else if users.len() > 1 {
        return Ok(GurtResponse::internal_server_error().with_string_body(String::from("Multiple users found - this shouldn't happen. Contact an admin.")));
    }
    Ok(GurtResponse::ok().with_string_body(""))
}

pub fn post(body: String) -> Result<GurtResponse> {
    let body = serde_json::from_str::<User>(&body);
    if (body.is_err()) {
        return Ok(GurtResponse::bad_request().with_string_body(String::from("Bad request")));
    }
    let body = body.unwrap();
    let db = Connection::open("gambling.db").unwrap();
    if (!body.create) {
        // Updates an existing user
        let query = format!("UPDATE users SET currency = {} WHERE name = '{}' AND password = '{}'", body.currency, body.name, body.password);
        db.execute(query).unwrap();
    }
    let query = format!(
        "INSERT INTO users (name, password, currency) VALUES ('{}', '{}', '{}')",
        body.name, body.password, body.currency
    );
    db.execute(query).unwrap();
    Ok(GurtResponse::ok().with_string_body(String::from("User created")))
}

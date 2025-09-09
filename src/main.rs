use gurt::prelude::*;
mod routes;
use sqlite::Connection;
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let db = Connection::open("gambling.db").unwrap();
    let query = "CREATE TABLE IF NOT EXISTS users (
        name TEXT NOT NULL,
        password TEXT NOT NULL,
        currency INTEGER NOT NULL
    )";

    db.execute(query).unwrap();
    let server = GurtServer::with_tls_certificates("cert.pem", "key.pem")?
        .get("/", |_ctx| async { routes::index::get() })
        .get("/slots", |_ctx| async { routes::slots::get() })
        .get("/users", |_ctx| async {
            let user_data = _ctx.request.text().unwrap();
            routes::users::get(user_data.clone())
        })
        .post("/users", |_ctx| async {
            let user_data = _ctx.request.text().unwrap();
            routes::users::post(user_data.clone())
        });

    println!("GURT server starting on gurt://127.0.0.1:4878");
    server.listen("127.0.0.1:4878").await
}

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
        .get("/login", |_ctx| async { routes::login::get() })
        .get("/blackjack", |_ctx| async { routes::blackjack::get() })
        .get("/roulette", |_ctx| async { routes::roulette::get() })
        .get("/poker", |_ctx| async { routes::poker::get() })
        .get("/dice", |_ctx| async { routes::dice::get() })
        .get("/coinflip", |_ctx| async { routes::coinflip::get() });

    println!("GURT server starting on gurt://127.0.0.1:4878");
    server.listen("127.0.0.1:4878").await
}

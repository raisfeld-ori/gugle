use gurt::prelude::*;
mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    let server = GurtServer::with_tls_certificates("cert.pem", "key.pem")?
        .get("/", |_ctx| async { routes::index()})
        .get("/slots", |_ctx| async { routes::slots() });

    println!("GURT server starting on gurt://127.0.0.1:4878");
    server.listen("127.0.0.1:4878").await
}
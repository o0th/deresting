use actix_web::{get, web, App, HttpServer, Responder};
use serde_json::{json};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const SERVICE: &str = env!("CARGO_PKG_NAME");

#[get("/")]
async fn version() -> impl Responder {
    web::Json(json!({
        "service": String::from(SERVICE),
        "version": String::from(VERSION)
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(version))
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

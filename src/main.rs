use actix_web::{
    middleware::Logger,
    get, App, HttpResponse, HttpServer, Responder,
};
use serde_json::json;

#[get("/api/balance")]
async fn balance_handler() -> impl Responder {
    const MESSAGE: &str = "Build simple CRUD API with Rust, SQLX, Postgres, and Actix Web";

    HttpResponse::Ok().json(json!({"status": "success", "message": MESSAGE}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        App::new()
            .service(balance_handler)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
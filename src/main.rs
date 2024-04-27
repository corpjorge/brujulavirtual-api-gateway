use actix_web::{App, HttpServer};

mod auth;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .configure(auth::configure)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
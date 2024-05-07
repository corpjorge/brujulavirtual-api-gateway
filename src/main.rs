mod controllers;
mod services;
mod api_gateway;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    api_gateway::server::run_server().await
}

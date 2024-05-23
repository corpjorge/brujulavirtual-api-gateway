use std::env;
use std::sync::Arc;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use reqwest::Client;

use crate::api_gateway::modules::setup_auth_service;
use crate::api_gateway::router::configure_routes;

pub async fn run_server() -> std::io::Result<()> {
    dotenv().ok();

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let server_address = format!("0.0.0.0:{}", port);
    let client = Arc::new(Client::new());

    println!("API Gateway running at http://{}", server_address);

    HttpServer::new(move || {
        App::new()
            .app_data(setup_auth_service(client.clone()).clone())
            .configure(configure_routes)
    })
    .bind(server_address)?
    .run()
    .await
}

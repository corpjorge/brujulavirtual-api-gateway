use std::env;
use std::sync::Arc;
use dotenv::dotenv;
use reqwest::Client;
use actix_web::{App, HttpServer, web};
use crate::services::auth_service::AuthService;
use crate::api_gateway::router::configure_routes;

pub async fn run_server() -> std::io::Result<()> {

    dotenv().ok();

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let server_address = format!("0.0.0.0:{}", port);


    let auth_service_url = env::var("AUTH_SERVICE_URL").unwrap_or_else(|_| "http://localhost:8081".to_string());

    let client = Arc::new(Client::new());

    let auth_service = web::Data::new(AuthService::new(auth_service_url, client.clone()));

    println!("API Gateway running at http://{}", server_address);

    HttpServer::new(move || {
        App::new()
            .app_data(auth_service.clone())
            .configure(configure_routes)
    })
        .bind(server_address)?
        .run()
        .await
}

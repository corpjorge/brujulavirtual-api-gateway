use actix_web::{App, HttpServer, web};
use reqwest::Client;
use std::sync::Arc;
use crate::services::auth_service::AuthService;
use crate::api_gateway::router::configure_routes;

pub async fn run_server() -> std::io::Result<()> {

    let client = Arc::new(Client::new());

    let auth_service = web::Data::new(AuthService::new(
        "http://localhost:8081".to_string(),
        client.clone(),
    ));

    let bind_address = "0.0.0.0:8080";

    HttpServer::new(move || {
        App::new()
            .app_data(auth_service.clone())
            .configure(configure_routes)
    })
        .bind(bind_address)?
        .run()
        .await
}

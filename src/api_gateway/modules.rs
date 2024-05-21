use std::sync::Arc;

use actix_web::web;
use reqwest::Client;

use crate::api_gateway::config::url_auth_service;
use crate::common::axios::Axios;
use crate::services::auth_service::AuthService;

pub fn setup_auth_service(client: Arc<Client>) -> web::Data<AuthService> {
    web::Data::new(AuthService::new(Arc::new(Axios::new(url_auth_service(), client.clone()))))
}

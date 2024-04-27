use actix_web::web;

pub use application::jwt_service::JwtService;
pub use domain::ports::auth_service::AuthService;
pub use infrastructure::repositories::ms_auth_repository::MsAuthRepository;
use crate::auth::infrastructure::controllers::auth_controller::auth_controller;

pub mod application;
pub mod domain;
pub mod infrastructure;

pub fn configure(cfg: &mut web::ServiceConfig) {
    let repository = MsAuthRepository::new();
    let auth_service = JwtService::new(repository);
    cfg.app_data(web::Data::new(auth_service));
    cfg.service(auth_controller);
}

use actix_web::{HttpResponse, Responder, web};
use crate::services::auth_service::AuthService;

#[derive(serde::Deserialize)]
pub struct LoginData {
    pub user: String,
    pub password: String,
}

#[derive(serde::Deserialize)]
pub struct RegisterData {
    pub user: String,
    pub password: String,
    pub email: String,
}

pub async fn login(service: web::Data<AuthService>, data: web::Json<LoginData>) -> impl Responder {
    let user = &data.user;
    let password = &data.password;

    if service.validate_login(user, password).await {
        HttpResponse::Ok().body("Inicio de sesión exitoso.")
    } else {
        HttpResponse::Unauthorized().body("Credenciales incorrectas.")
    }
}
use std::sync::{Arc, Mutex};
use actix_web::{HttpResponse, post, Responder, web};
use serde::{Deserialize, Serialize};
use crate::auth::AuthService;

#[derive(Serialize)]
struct ValidationResponse {
    user: String,
    password: String,
    result: String,
}

#[derive(Deserialize)]
struct UserData {
    user: String,
    password: String,
}

#[post("/validate")]
pub async fn auth_controller(data: web::Json<UserData>, auth_service: web::Data<Arc<Mutex<dyn AuthService>>>) -> impl Responder {
    let user = &data.user;
    let password = &data.password;

    println!("Validating data - User: {}, Password: {}", user, password);

    let auth_service_lock = auth_service.lock();
    match auth_service_lock {
        Ok(service) => {
            let result = service.validate(user.clone());
            println!("Validation Result: {:?}", result);
        },
        Err(e) => {
            println!("Error al obtener el lock del servicio de autenticación: {:?}", e);
            return HttpResponse::InternalServerError().json("Error interno del servidor");
        }
    }


    let response = ValidationResponse {
        user: user.clone(),
        password: password.clone(),
        result: "success".to_string(),
    };

    HttpResponse::Ok().json(response)

}

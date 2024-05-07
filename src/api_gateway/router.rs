use actix_web::web;
use crate::controllers::{auth_controller};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::resource("/auth").route(web::post().to(auth_controller::login)));
}

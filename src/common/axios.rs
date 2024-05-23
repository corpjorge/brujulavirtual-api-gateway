use actix_web::{
    http::header::{HeaderName, HeaderValue},
    HttpResponse,
};
use log::info;
use reqwest::Client;
use serde::Serialize;
use std::sync::Arc;

pub struct Axios {
    client: Arc<Client>,
    pub(crate) base_url: String,
}

impl Axios {
    pub fn new(base_url: String, client: Arc<Client>) -> Self {
        Axios { client, base_url }
    }

    pub(crate) async fn post<T: Serialize>(
        &self,
        url: &str,
        data: &T,
    ) -> Result<HttpResponse, HttpResponse> {
        let response = match self.client.post(url).json(data).send().await {
            Ok(resp) => resp,
            Err(err) => {
                info!("ERROR: {:?}", err);
                if let Some(status) = err.status() {
                    let body = err.to_string();
                    return Err(HttpResponse::build(
                        actix_web::http::StatusCode::from_u16(status.as_u16()).unwrap(),
                    )
                    .body(body));
                }
                return Err(HttpResponse::InternalServerError().finish());
            }
        };

        let status = response.status();
        let mut actix_response =
            HttpResponse::build(actix_web::http::StatusCode::from_u16(status.as_u16()).unwrap());

        for (key, value) in response.headers().iter() {
            let header_name = HeaderName::from_bytes(key.as_str().as_bytes()).unwrap();
            let header_value = HeaderValue::from_bytes(value.as_bytes()).unwrap();
            actix_response.insert_header((header_name, header_value));
        }

        let body = match response.bytes().await {
            Ok(bytes) => bytes,
            Err(_) => return Err(HttpResponse::InternalServerError().finish()),
        };

        Ok(actix_response.body(body))
    }
}

use std::sync::Arc;
use actix_web::{http::header::{HeaderName, HeaderValue}, HttpResponse};
use log::info;
use reqwest::Client;

pub struct AuthService {
    client: Arc<Client>,
    base_url: String,
}

#[derive(serde::Serialize, Debug)]
struct LoginData<'a> {
    user: &'a str,
    password: &'a str,
}

impl AuthService {
    pub fn new(base_url: String, client: Arc<Client>) -> Self {
        AuthService { client, base_url }
    }

    pub async fn validate_login(&self, user: &str, password: &str) -> Result<HttpResponse, HttpResponse> {
        let url = format!("{}/auth", self.base_url);

        let login_data = LoginData {
            user,
            password,
        };

        let response = match self.client.post(&url)
            .json(&login_data)
            .send()
            .await {
            Ok(resp) => resp,
            Err(err) => {
                info!("ERROR: {:?}", err);
                if let Some(status) = err.status() {
                    let body = err.to_string();
                    return Err(HttpResponse::build(actix_web::http::StatusCode::from_u16(status.as_u16()).unwrap()).body(body));
                }
                return Err(HttpResponse::InternalServerError().finish());
            }
        };

        let status = response.status();
        let mut actix_response = HttpResponse::build(actix_web::http::StatusCode::from_u16(status.as_u16()).unwrap());

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

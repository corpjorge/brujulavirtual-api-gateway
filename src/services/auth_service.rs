use reqwest::Client;
use std::sync::Arc;
use log::info;

pub struct AuthService {
    client: Arc<Client>,
    base_url: String,
}

impl AuthService {
    pub fn new(base_url: String, client: Arc<Client>) -> Self {
        AuthService { client, base_url }
    }

    pub async fn validate_login(&self, user: &str, password: &str) -> bool {
        let url = format!("{}/auth", self.base_url);

        println!("Solicitando URL: {}", url);
        info!("Solicitando URL: {}", url);

        let login_data = LoginData {
            user,
            password,
        };

        info!("DATOS: {:?}", login_data);

        let response = self.client.post(&url)
            .json(&login_data)
            .send()
            .await;

        response.map_or(false, |resp| resp.status().is_success())
    }
}

#[derive(serde::Serialize, Debug)]
struct LoginData<'a> {
    user: &'a str,
    password: &'a str,
}

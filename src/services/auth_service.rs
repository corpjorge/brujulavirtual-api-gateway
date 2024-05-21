use std::sync::Arc;

use actix_web::HttpResponse;

use crate::common::axios::Axios;

pub struct AuthService {
    axios: Arc<Axios>,
}

#[derive(serde::Serialize, Debug)]
struct LoginData<'a> {
    user: &'a str,
    password: &'a str,
}

impl AuthService {
    pub fn new(axios: Arc<Axios>) -> Self {
        AuthService { axios }
    }

    pub async fn validate_login(&self, user: &str, password: &str) -> Result<HttpResponse, HttpResponse> {
        let url = format!("{}/auth", self.axios.base_url);

        let login_data = LoginData {
            user,
            password,
        };

        self.axios.post(&url, &login_data).await
    }
}

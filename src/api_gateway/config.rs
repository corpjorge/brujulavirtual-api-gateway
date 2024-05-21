use std::env;

pub fn url_auth_service() -> String {
    env::var("AUTH_SERVICE_URL").unwrap_or_else(|_| "http://localhost:8081".to_string())
}
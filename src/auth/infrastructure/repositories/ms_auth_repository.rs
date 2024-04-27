use crate::auth::domain::ports::auth_repository::AuthRepository;

pub struct MsAuthRepository {}

impl MsAuthRepository {
    pub fn new() -> Self {
        MsAuthRepository {}
    }
}

impl AuthRepository for MsAuthRepository {
    fn validate(&self, data: String) -> String {
        println!("Datos: {:?}", data);
        return data;
    }
}
use crate::auth::domain::ports::auth_repository::AuthRepository;
use crate::auth::domain::ports::auth_service::AuthService;

pub struct JwtService<T: AuthRepository> {
    repository: T,
}

impl<T: AuthRepository> JwtService<T> {
    pub fn new(repository: T) -> Self {
        JwtService { repository }
    }
}

impl<T: AuthRepository> AuthService for JwtService<T> {
    fn validate(&self, data: String) -> String {
        println!("xxx");
        self.repository.validate(data)
    }
}
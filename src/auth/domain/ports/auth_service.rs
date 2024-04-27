pub trait AuthService {
    fn validate(&self, data: String) -> String;
}

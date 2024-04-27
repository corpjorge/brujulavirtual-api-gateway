pub trait AuthRepository {
    fn validate(&self, data: String) -> String;
}

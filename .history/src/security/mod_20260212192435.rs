#[derive(Clone)]
pub struct SecurityManager {
    expected_token: String,
}

impl SecurityManager {
    pub fn new(token: String) -> Self {
        Self { expected_token: token }
    }

    pub fn validate_token(&self, token: &str) -> bool {
        self.expected_token == token
    }
}

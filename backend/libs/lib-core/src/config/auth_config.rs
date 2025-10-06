use std::env;

#[derive(Debug, Clone)]
pub struct AuthConfig {
    pub token_duration_days: i64,
    pub password_min_length: usize,
}

impl AuthConfig {
    pub fn from_env() -> Self {
        Self {
            token_duration_days: env::var("AUTH_TOKEN_DURATION_DAYS")
                .unwrap_or_else(|_| "7".to_string())
                .parse()
                .unwrap_or(7),
            password_min_length: env::var("PASSWORD_MIN_LENGTH")
                .unwrap_or_else(|_| "8".to_string())
                .parse()
                .unwrap_or(8),
        }
    }
}

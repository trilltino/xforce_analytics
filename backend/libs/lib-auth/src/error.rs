use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Password hashing failed: {0}")]
    HashingFailed(String),

    #[error("Token generation failed: {0}")]
    TokenGenerationFailed(String),

    #[error("Token validation failed: {0}")]
    TokenValidationFailed(String),

    #[error("Session expired")]
    SessionExpired,

    #[error("User not found")]
    UserNotFound,

    #[error("{0}")]
    Custom(String),
}

pub type AuthResult<T> = Result<T, AuthError>;

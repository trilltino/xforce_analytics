use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use bcrypt::{hash, verify, DEFAULT_COST};

use crate::error::{AuthError, AuthResult};

#[derive(Debug, Clone, Copy)]
pub enum HashScheme {
    Argon2,
    Bcrypt,
}

impl Default for HashScheme {
    fn default() -> Self {
        Self::Argon2
    }
}

/// Hash a password using the specified scheme
pub fn hash_password(password: &str, scheme: HashScheme) -> AuthResult<String> {
    match scheme {
        HashScheme::Argon2 => hash_argon2(password),
        HashScheme::Bcrypt => hash_bcrypt(password),
    }
}

/// Verify a password against a hash (auto-detects scheme)
pub fn verify_password(password: &str, hash: &str) -> AuthResult<bool> {
    // Try Argon2 first (starts with $argon2)
    if hash.starts_with("$argon2") {
        verify_argon2(password, hash)
    }
    // Try Bcrypt (starts with $2)
    else if hash.starts_with("$2") {
        verify_bcrypt(password, hash)
    }
    else {
        Err(AuthError::Custom("Unknown hash scheme".to_string()))
    }
}

// Argon2 implementation (recommended)
fn hash_argon2(password: &str) -> AuthResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| AuthError::HashingFailed(e.to_string()))
}

fn verify_argon2(password: &str, hash: &str) -> AuthResult<bool> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| AuthError::Custom(format!("Invalid hash format: {}", e)))?;

    let argon2 = Argon2::default();

    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

// Bcrypt implementation (alternative)
fn hash_bcrypt(password: &str) -> AuthResult<String> {
    hash(password, DEFAULT_COST)
        .map_err(|e| AuthError::HashingFailed(e.to_string()))
}

fn verify_bcrypt(password: &str, hash: &str) -> AuthResult<bool> {
    verify(password, hash)
        .map_err(|e| AuthError::Custom(format!("Bcrypt verification failed: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_argon2_hash_and_verify() {
        let password = "test_password_123";
        let hash = hash_password(password, HashScheme::Argon2).unwrap();

        assert!(verify_password(password, &hash).unwrap());
        assert!(!verify_password("wrong_password", &hash).unwrap());
    }

    #[test]
    fn test_bcrypt_hash_and_verify() {
        let password = "test_password_123";
        let hash = hash_password(password, HashScheme::Bcrypt).unwrap();

        assert!(verify_password(password, &hash).unwrap());
        assert!(!verify_password("wrong_password", &hash).unwrap());
    }
}

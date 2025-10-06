use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;


/// Generate a secure random token
pub fn generate_token(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

/// Generate a secure session token (64 characters)
pub fn generate_session_token() -> String {
    generate_token(64)
}

/// Hash a token using SHA-256 (for storage)
pub fn hash_token(token: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Verify a token against its hash
pub fn verify_token(token: &str, hash: &str) -> bool {
    hash_token(token) == hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_token() {
        let token1 = generate_token(32);
        let token2 = generate_token(32);

        assert_eq!(token1.len(), 32);
        assert_eq!(token2.len(), 32);
        assert_ne!(token1, token2); // Should be different
    }

    #[test]
    fn test_hash_and_verify_token() {
        let token = generate_session_token();
        let hash = hash_token(&token);

        assert!(verify_token(&token, &hash));
        assert!(!verify_token("wrong_token", &hash));
    }
}

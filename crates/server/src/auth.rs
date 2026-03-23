use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, SaltString},
    Argon2, PasswordHasher, PasswordVerifier,
};

/// Hash a password using Argon2id. Returns the PHC-format hash string.
/// IMPORTANT: This is CPU-intensive. Call via tokio::task::spawn_blocking in async contexts.
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
}

/// Verify a password against its Argon2id PHC hash.
/// Returns true if the password matches, false otherwise.
/// IMPORTANT: This is CPU-intensive. Call via tokio::task::spawn_blocking in async contexts.
pub fn verify_password(password: &str, hash: &str) -> bool {
    PasswordHash::new(hash)
        .and_then(|parsed| Argon2::default().verify_password(password.as_bytes(), &parsed))
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_password_produces_argon2id_format() {
        let hash = hash_password("testpassword").expect("hash should succeed");
        assert!(hash.starts_with("$argon2id$"), "hash should be in PHC format with $argon2id$");
    }

    #[test]
    fn verify_password_returns_true_for_correct_password() {
        let password = "correctpassword";
        let hash = hash_password(password).expect("hash should succeed");
        assert!(verify_password(password, &hash), "correct password should verify");
    }

    #[test]
    fn verify_password_returns_false_for_incorrect_password() {
        let hash = hash_password("correctpassword").expect("hash should succeed");
        assert!(!verify_password("wrongpassword", &hash), "wrong password should not verify");
    }

    #[test]
    fn hash_password_produces_unique_hashes_for_same_password() {
        let password = "samepassword";
        let hash1 = hash_password(password).expect("hash 1 should succeed");
        let hash2 = hash_password(password).expect("hash 2 should succeed");
        assert_ne!(hash1, hash2, "same password should produce different hashes due to unique salts");
    }
}

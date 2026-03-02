use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use password_hash::{PasswordHash, SaltString};
use rand_core::OsRng;

pub fn hash_password(password: &str) -> Result<String, password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(hash.to_string())
}

pub fn verify_password(password: &str, password_hash_str: &str) -> bool {
    if let Ok(parsed) = PasswordHash::new(password_hash_str) {
        let argon2 = Argon2::default();
        argon2.verify_password(password.as_bytes(), &parsed).is_ok()
    } else {
        false
    }
}

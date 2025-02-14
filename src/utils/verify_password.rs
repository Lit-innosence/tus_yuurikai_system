use argon2::password_hash;
use argon2::{Argon2, PasswordHash, PasswordVerifier};

pub fn verify_password_hash(password: String, expected_password_hash: String) -> Result<(), password_hash::Error> {
    let expected_password_hash = PasswordHash::new(expected_password_hash.as_str())?;
    Argon2::default().verify_password(password.as_bytes(), &expected_password_hash)
}
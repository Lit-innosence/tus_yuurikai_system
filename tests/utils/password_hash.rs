use argon2::password_hash::{self, SaltString};
use argon2::{Argon2, PasswordHasher, Algorithm, Version, Params};


pub fn compute_password_hash(password: String) -> Result<String, password_hash::Error> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let password_hash = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None)?,
    ).hash_password(password.as_bytes(), &salt)?.to_string();

    Ok(password_hash)
}
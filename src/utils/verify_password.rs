use argon2::password_hash;
use argon2::{Argon2, PasswordHash, PasswordVerifier};

/// ### verify_password_hash
/// パスワードをハッシュ値と比較検証する
///
/// password                : 検証するパスワード
///
/// expected_password_hash  : 比較するハッシュ値
pub fn verify_password_hash(password: String, expected_password_hash: String) -> Result<(), password_hash::Error> {
    let expected_password_hash = PasswordHash::new(expected_password_hash.as_str())?;
    Argon2::default().verify_password(password.as_bytes(), &expected_password_hash)
}
use crate::adapters::controller::Claims;
use jsonwebtoken::{decode, Validation, DecodingKey};
use std::env;
use dotenv::dotenv;

/// ### jwt検証
pub fn decode_jwt(jwt: &String) -> Option<Claims> {

    let validation = Validation::default();

    dotenv().ok();
    let secret = env::var("TOKEN_KEY").expect("token key must be set");

    match decode::<Claims>(&jwt, &DecodingKey::from_secret(secret.as_ref()), &validation) {
        Ok(token) => Option::Some(token.claims),
        _ => Option::None,
    }
}
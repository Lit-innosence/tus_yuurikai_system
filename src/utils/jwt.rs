use jsonwebtoken::{encode, decode, Header, Algorithm, EncodingKey, DecodingKey, Validation};
use chrono::{Utc, TimeDelta};
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::env;

/// ### Claims
/// JWTペイロードに指定する構造体
///
/// subject     : tokenの持ち主
///
/// expire      : tokenの持続時間
///
/// issued at   : tokenの発行時刻
#[derive(Serialize, Deserialize)]
pub struct Claims{
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

/// ### encode_jwt
/// JWTを発行する
///
/// username    : jwtの持ち主
///
/// exp         : jwtの持続時間
///
/// key         : jwtの鍵
pub fn encode_jwt(username: &String, exp: TimeDelta, key: &String) -> String {

    // headerの宣言
    let mut header = Header::default();

    // 使用するトークンはjwt
    header.typ = Some("JWT".to_string());

    // 使用するアルゴリズムはHMAC SHA-256
    header.alg = Algorithm::HS256;

    // 現在時刻を取得
    let now = Utc::now();

    // claimsを設定
    let admin_claims = Claims{
        sub: username.clone(),
        exp: (now + exp).timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    // jwtを発行
    return encode(&header, &admin_claims, &EncodingKey::from_secret(key.as_ref())).unwrap()
}

/// ### decode_jwt
/// JWTを検証する
///
/// jwt     : 検証するjwt
pub fn decode_jwt(jwt: &String) -> Option<Claims> {

    let validation = Validation::default();

    dotenv().ok();
    let secret = env::var("TOKEN_KEY").expect("token key must be set");

    match decode::<Claims>(&jwt, &DecodingKey::from_secret(secret.as_ref()), &validation) {
        Ok(token) => Option::Some(token.claims),
        _ => Option::None,
    }
}
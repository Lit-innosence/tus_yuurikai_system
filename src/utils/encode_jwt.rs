use jsonwebtoken::{encode, Header, Algorithm, EncodingKey};
use chrono::{Utc, TimeDelta};
use crate::adapters::controller::Claims;


pub fn encode_jwt(username: &String, exp: TimeDelta, key: &String) -> String {
    // jwtの発行

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
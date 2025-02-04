use jsonwebtoken::{encode, Header, Algorithm, EncodingKey};
use chrono::{Utc, TimeDelta};
use serde::{Deserialize, Serialize};

/// ### JWTペイロードに指定する構造体
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
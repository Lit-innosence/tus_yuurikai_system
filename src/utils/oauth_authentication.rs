use std::{collections::HashMap, env, time::Duration, vec};
use serde::{Serialize, Deserialize};
use dotenv::dotenv;
use jsonwebtoken::{encode, Header, Algorithm, EncodingKey};
use chrono::Utc;

/// ### Token
/// アクセストークンを取得するリクエストのレスポンス
#[derive(Debug, Deserialize, Serialize)]
pub struct Token {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    // pub scope: String,
}

/// ### Token
/// アクセストークンを取得するリクエストのエラーレスポンス
#[derive(Debug, Deserialize)]
pub struct TokenError {
    pub error: String,
    pub error_description: String,
}

/// ### Claim
/// OAuth認証に使用するJWTのClaim
#[derive(Debug, Serialize)]
pub struct Claim {
    iss: String,
    scope: String,
    aud: String,
    sub: Option<String>,
    exp: i64,
    iat: i64,
}

/// ### get_access_token
///
pub async fn get_access_token() -> Result<Token, TokenError> {

    //JWTの作成
    // headerの作成
    let mut header = Header::default();

    // 使用するTokenはJWT
    header.typ = Some("JWT".to_string());

    // 使用するTokenはRSA-SHA256
    header.alg = Algorithm::RS256;

    // 現在時刻の取得
    let now = Utc::now();

    // アカウントのメールアドレスの取得
    let service_account = env::var("CLIENT_EMAIL").expect("client email address must be set.");

    // scopeの取得
    let scope = env::var("SCOPE").expect("scope must be set.");

    // リクエストを送信するURIを取得
    let auth_uri = env::var("AUTH_URI").expect("auth URI must be set");

    // 秘密鍵を取得
    let private_key = env::var("OAUTH_PRIVATE_KEY").expect("oauth private key must be set.");

    // Claimの作成
    let claim = Claim {
        iss: service_account,
        scope: scope,
        aud: auth_uri.clone(),
        sub: None,
        exp: now.timestamp() + 3600,
        iat: now.timestamp(),
    };

    println!("{:?}", claim);

    // JWTの発行
    let jwt = encode(&header, &claim, &EncodingKey::from_rsa_pem(private_key.as_bytes()).unwrap()).unwrap();

    // リクエストのパラメータ作成
    let mut params = HashMap::new();

    params.insert("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer");
    params.insert("assertion", jwt.as_str());

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(60))
        .build().unwrap();

    let result = client.post(auth_uri)
        .form(&params)
        .send()
        .await
        .unwrap();
    let status = result.status();
    println!("{}", status);

    if status.is_success() {
        let text = result.text().await.unwrap();
        println!("{}", text);
        let token: Token = serde_json::from_str(text.as_str()).unwrap();
        Ok(token)
    } else {
        let err = result.json::<TokenError>().await.unwrap();
        println!("{}\n{}", err.error, err.error_description);
        Err(err)
    }
}

/// ### RefreshResult
/// アクセストークンを取得するリクエストのレスポンスです
///
/// access_token    : アクセストークン
///
/// expires_in      : トークンの使用期限（秒）
///
/// scope           : access_tokenに寄って付与されるスコープ
///
/// token_type      : 常にBearerが指定される
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct RefreshResult {
    pub access_token: String,
    pub expires_in: u32,
    pub scope: String,
    pub token_type: String,
}

/// ### refresh_access_token
/// リフレッシュトークンを使用して、Google Oauthのアクセストークンを取得します.
///
/// refresh_token   : リフレッシュトークン
///
/// client_id       : Oauth Client Id
///
/// client_secret   : Oauth Client Secret
pub async fn refresh_access_token(refresh_token: &str, client_id: &str, client_secret: &str) -> Result<RefreshResult, Box<dyn std::error::Error + Send + Sync>> {
    let mut params = HashMap::new();

    params.insert("refresh_token", refresh_token);
    params.insert("client_id", client_id);
    params.insert("client_secret", client_secret);
    params.insert("grant_type", "refresh_token");

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(60))
        .build()?;

    dotenv().ok();
    let oauth_uri = env::var("OAUTH_URI").expect("oauth uri must be set.");

    let result = client.post(oauth_uri)
        .form(&params)
        .send()
        .await?
        .json::<RefreshResult>()
        .await?;

    Ok(result)
}
use std::{collections::HashMap, time::Duration, env};
use serde::{Serialize, Deserialize};
use dotenv::dotenv;

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
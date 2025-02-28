use reqwest;
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
pub struct RecaptchaResponse {
    pub success: bool,
    #[serde(default)]
    pub score: Option<f64>,
    #[serde(default)]
    pub action: Option<String>,
    pub challenge_ts: Option<String>,
    pub hostname: Option<String>,
    #[serde(rename = "error-codes")]
    pub error_codes: Option<Vec<String>>,
}


pub async fn verify_recaptcha(recaptcha_token: &str) -> Result<bool, reqwest::Error> {
    let recaptcha_secret = env::var("RECAPTCHA_SECRET_KEY").expect("RECAPTCHA_SECRET_KEY must be set.");

    let client = reqwest::Client::new();
    let params = [("secret", recaptcha_secret), ("response", recaptcha_token.to_string())];
    let verification_response = client
        .post("https://www.google.com/recaptcha/api/siteverify")
        .form(&params)
        .send()
        .await?;

    if verification_response.status() != reqwest::StatusCode::OK {
        return Ok(false);
    }

    let recaptcha_result: RecaptchaResponse = verification_response.json().await?;
    
    Ok(recaptcha_result.success 
        && recaptcha_result.score.unwrap_or(0.0) >= 0.5 
        && recaptcha_result.action.as_deref() == Some("confirm_page"))
}

use std::sync::Arc;
use std::env;
use crate::domain::student::UserInfo;
use crate::adapters::repository::AuthRepository;
use crate::infrastructure::models::Auth;
use crate::utils::token::generate_token;

use dotenv::dotenv;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rocket::http::Status;
use async_trait::async_trait;
use diesel::result::Error;

pub struct AuthUsecaseImpl {
     pub auth_repository: Arc<dyn AuthRepository>,
}

#[async_trait]
pub trait AuthUsecase: Sync + Send {
    async fn register(&self, main_user: &UserInfo, co_user: &UserInfo, is_same: bool) -> Result<Auth, Error>;
    async fn mail_sender(&self, user_address: String, content: String, subject: &str) -> Result<(), Status>;
    async fn token_check(&self, token: String, is_main: bool) -> Result<Auth, Status>;
    async fn update_phase(&self, token: String, phase: i32) -> Result<usize, Status>;
    async fn delete(&self, token: String) -> Result<usize, Status>;
}

impl AuthUsecaseImpl {
    pub fn new(auth_repository: Arc<dyn AuthRepository>) -> Self {
        AuthUsecaseImpl { auth_repository }
    }
}

#[async_trait]
impl AuthUsecase for AuthUsecaseImpl {
    // tokenの生成、DBへの登録
    async fn register(&self, main_user: &UserInfo, co_user: &UserInfo, is_same: bool) -> Result<Auth, Error> {
        let main_token = generate_token();
        let mut co_token = generate_token();
        if is_same {
            co_token.clone_from(&main_token);
        }
        self.auth_repository.insert(&main_token, &main_user.student_id, &main_user.family_name, &main_user.given_name, &co_token, &co_user.student_id, &co_user.family_name, &co_user.given_name).await
    }
    async fn mail_sender(&self, user_address: String, content: String, subject: &str) -> Result<(), Status> {
        // 環境変数の読み取り
        dotenv().ok();
        let sender_address = env::var("SENDER_MAIL_ADDRESS").expect("SENDER_MAIL_ADDRESS must be set.");
        let appkey = env::var("MAIL_APP_KEY").expect("MAIL_APP_KEY must be set.");

        let email = Message::builder()
            .from(
                format!("Developer <{}>", sender_address)
                    .parse()
                    .map_err(|_| Status::InternalServerError)?,
            )
            .to(format!("User <{}>", user_address)
                .parse()
                .map_err(|_| Status::InternalServerError)?)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(content)
            .map_err(|_| Status::InternalServerError)?;

        let creds = Credentials::new(sender_address.to_owned(), appkey.to_owned());

        // Gmailにsmtp接続する
        let mailer = SmtpTransport::relay("smtp.gmail.com")
            .map_err(|_| Status::InternalServerError)?
            .credentials(creds)
            .build();

        // メール送信
        mailer.send(&email).map_err(|_| Status::InternalServerError)?;

        Ok(())
    }
    async fn token_check(&self, token: String, is_main: bool) -> Result<Auth, Status> {
        let auth = match self.auth_repository.get_by_token(&token).await {
            Ok(auth) => auth,
            Err(_) => return Err(Status::Unauthorized),
        };
        if (is_main && auth.main_auth_token == token) || (!is_main && auth.co_auth_token == token) {
            Ok(auth)
        } else {
            Err(Status::Unauthorized)
        }
    }
    async  fn update_phase(&self, token: String, phase: i32) -> Result<usize, Status> {
        match self.auth_repository.update_phase(&token, &phase).await {
            Ok(ok) => Ok(ok),
            Err(_) => return Err(Status::InternalServerError),
        }
    }
    async  fn delete(&self, token: String) -> Result<usize, Status> {
        match self.auth_repository.delete(&token).await {
            Ok(ok) => Ok(ok),
            Err(_) => return Err(Status::InternalServerError),
        }
    }
}
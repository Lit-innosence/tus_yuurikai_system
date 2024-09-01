use std::sync::Arc;
use std::env;
use crate::domain::student::UserInfo;
use crate::adapters::repository::AuthRepository;
use crate::infrastracture::models::Auth;
use crate::utils::token::generate_token;

use dotenv::dotenv;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rocket::http::Status;
use async_trait::async_trait;

pub struct AuthUsecaseImpl {
    auth_repository: Arc<dyn AuthRepository>,
}

#[async_trait]
pub trait AuthUsecase: Sync + Send {
    async fn mail_sender(&self, student: &UserInfo) -> Result<Auth, Status>;
}

impl AuthUsecaseImpl {
    pub fn new(auth_repository: Arc<dyn AuthRepository>) -> Self {
        AuthUsecaseImpl { auth_repository }
    }
}

#[async_trait]
impl AuthUsecase for AuthUsecaseImpl {
    async fn mail_sender(&self, student: &UserInfo) -> Result<Auth, Status> {
        // 環境変数の読み取り
        dotenv().ok();
        let sender_address = env::var("SENDER_MAIL_ADDRESS").expect("SENDER_MAIL_ADDRESS must be set.");
        let appkey = env::var("MAIL_APP_KEY").expect("MAIL_APP_KEY must be set.");
        let app_url = env::var("APP_URL").expect("APP_URL must be set.");

        // tokenの生成
        let token = generate_token();

        let auth = self.auth_repository.insert(&token, &student.student_id, &student.family_name, &student.given_name).await.map_err(|_| Status::InternalServerError)?;

        // メール内容の作成
        let user_address = format!("{}@ed.tus.ac.jp", student.student_id);

        let content = format!("{}{} 様\n\n以下のURLにアクセスして認証を完了してください。\n{}/locker/user-register?token={}", student.family_name, student.given_name, app_url, token);

        let email = Message::builder()
            .from(
                format!("Developer <{}>", sender_address)
                    .parse()
                    .map_err(|_| Status::InternalServerError)?,
            )
            .to(format!("User <{}>", user_address)
                .parse()
                .map_err(|_| Status::InternalServerError)?)
            .subject("ロッカーシステム メール認証")
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

        Ok(auth)
    }
}
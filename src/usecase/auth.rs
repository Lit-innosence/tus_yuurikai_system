use std::sync::Arc;
use std::env;
use crate::domain::{student::UserInfo, circle::OrganizationInfo};
use crate::adapters::repository::{auth::AuthRepository, circle_auth_info::CircleAuthInfoRepository, locker_auth_info::LockerAuthInfoRepository};
use crate::infrastructure::models::{Auth, CircleAuthInfo, LockerAuthInfo};
use crate::utils::token::generate_token;

use aws_sdk_sesv2::error::DisplayErrorContext;
use aws_sdk_sesv2::types::{Message as awsMessage, Body, Content, Destination, EmailContent};
use aws_sdk_sesv2::Client;
use dotenv::dotenv;
use uuid::Uuid;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::{Message as lettreMessage, SmtpTransport, Transport};
use rocket::http::Status;
use async_trait::async_trait;

pub struct AuthUsecaseImpl {
     pub auth_repository: Arc<dyn AuthRepository>,
     pub locker_auth_info_repository: Arc<dyn LockerAuthInfoRepository>,
     pub circle_auth_info_repository: Arc<dyn CircleAuthInfoRepository>,
}

#[async_trait]
pub trait AuthUsecase: Sync + Send {
    async fn locker_register(&self, main_user: &UserInfo, co_user: &UserInfo, phase: &String, is_same: bool) -> Result<Auth, Status>;
    async fn circle_register(&self, organization: &OrganizationInfo, phase: &String, is_same: bool) -> Result<Auth, Status>;
    async fn mail_sender(&self, user_address: String, content: String, subject: &str) -> Result<(), Status>;
    async fn aws_mail_sender(&self, client: &Client, user_address: String, content: String, subject: &str) -> Result<(), Status>;
    async fn token_check(&self, token: String, is_main: bool) -> Result<Auth, Status>;
    async fn get_locker_auth_info(&self, auth_id: &Uuid) -> Result<LockerAuthInfo, Status>;
    async fn get_circle_auth_info(&self, auth_id:&Uuid) -> Result<CircleAuthInfo, Status>;
    async fn update_phase(&self, auth_id: &Uuid, phase: String) -> Result<usize, Status>;
    async fn delete(&self, auth_id: &Uuid) -> Result<usize, Status>;
}

impl AuthUsecaseImpl {
    pub fn new(auth_repository: Arc<dyn AuthRepository>, locker_auth_info_repository: Arc<dyn LockerAuthInfoRepository>, circle_auth_info_repository: Arc<dyn CircleAuthInfoRepository>) -> Self {
        AuthUsecaseImpl { auth_repository, locker_auth_info_repository, circle_auth_info_repository}
    }
}

#[async_trait]
impl AuthUsecase for AuthUsecaseImpl {
    // ロッカー用、tokenの生成、DBへの登録
    async fn locker_register(&self, main_user: &UserInfo, co_user: &UserInfo, phase: &String, is_same: bool) -> Result<Auth, Status> {
        let main_token = generate_token();
        let mut co_token = generate_token();
        if is_same {
            co_token.clone_from(&main_token);
        }
        let auth = match self.auth_repository.insert(&main_token, &co_token, phase).await {
            Ok(auth) => {auth},
            Err(_) => {return Err(Status::InternalServerError)}
        };

        match self.locker_auth_info_repository.insert(&auth.auth_id,
                                                    &main_user.student_id,
                                                    &main_user.family_name,
                                                    &main_user.given_name,
                                                    &co_user.student_id,
                                                    &co_user.family_name,
                                                    &co_user.given_name).await {
            Ok(_) => {return Ok(auth)},
            Err(_) => {return Err(Status::InternalServerError)},
        }
    }

    // 団体登録用、tokenの生成、DBへの登録
    async fn circle_register(&self, organization: &OrganizationInfo, phase: &String, is_same: bool) -> Result<Auth, Status> {
        let main_token = generate_token();
        let mut co_token = generate_token();

        if is_same {
            co_token.clone_from(&main_token);
        }
        let auth = match self.auth_repository.insert(&main_token, &co_token, phase).await {
            Ok(auth) => {auth},
            Err(_) => {return Err(Status::InternalServerError)}
        };

        match self.circle_auth_info_repository.insert(&auth.auth_id,
                                                    &organization.main_user.student_id,
                                                    &organization.main_user.family_name,
                                                    &organization.main_user.given_name,
                                                    &organization.main_user.email,
                                                    &organization.main_user.phone_number,
                                                    &organization.co_user.student_id,
                                                    &organization.co_user.family_name,
                                                    &organization.co_user.given_name,
                                                    &organization.co_user.email,
                                                    &organization.co_user.phone_number,
                                                    &organization.b_doc,
                                                    &organization.c_doc,
                                                    &organization.d_doc,
                                                    &organization.organization.organization_name,
                                                    &organization.organization.organization_ruby,
                                                    &organization.organization.organization_email).await {
            Ok(_) => {return Ok(auth)},
            Err(_) => {return Err(Status::InternalServerError)}
        }
    }

    async fn mail_sender(&self, user_address: String, content: String, subject: &str) -> Result<(), Status> {
        // 環境変数の読み取り
        dotenv().ok();
        let sender_address = env::var("SENDER_MAIL_ADDRESS").expect("SENDER_MAIL_ADDRESS must be set.");
        let appkey = env::var("MAIL_APP_KEY").expect("MAIL_APP_KEY must be set.");

        let email = lettreMessage::builder()
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

        // SMTP認証情報
        let creds = Credentials::new(sender_address.to_owned(), appkey.to_owned());

        // TLSパラメータを生成
        let tls_parameters = TlsParameters::builder("smtp.gmail.com".to_string())
        .build()
        .map_err(|_| Status::InternalServerError)?;

        // Gmailにsmtp接続する
        let mailer = SmtpTransport::relay("smtp.gmail.com")
            .map_err(|_| Status::InternalServerError)?
            .port(587)
            .tls(Tls::Required(tls_parameters))
            .credentials(creds)
            .build();

        // メール送信
        mailer.send(&email).map_err(|_| Status::InternalServerError)?;

        Ok(())
    }

    async fn aws_mail_sender(&self, client: &Client, user_address: String, content: String, subject: &str) -> Result<(), Status> {

        let cs: Vec<String> = vec![user_address];

        let mut dest : Destination = Destination::builder().build();
        dest.to_addresses = Some(cs);

        let subject_content = Content::builder()
            .data(subject)
            .charset("UTF-8")
            .build()
            .expect("building Content");
        let body_content = Content::builder()
            .data(content)
            .charset("UTF-8")
            .build()
            .expect("building Content");
        let body = Body::builder().text(body_content).build();

        let msg = awsMessage::builder()
            .subject(subject_content)
            .body(body)
            .build();

        let email_content = EmailContent::builder().simple(msg).build();

        dotenv().ok();

        let from = env::var("SENDER_EMAIL_ADDRESS").expect("SENDER_EMAIL_ADDRESS must be set.");

        match client
            .send_email()
            .from_email_address(from.as_str())
            .destination(dest)
            .content(email_content)
            .send()
            .await {
                Ok(_) => {},
                Err(err) => {
                    println!("Error: {}", DisplayErrorContext(&err));
                    return Err(Status::InternalServerError)},
            };

        println!("Email sent successfully");

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
    async fn get_locker_auth_info(&self, auth_id: &Uuid) -> Result<LockerAuthInfo, Status> {
        match self.locker_auth_info_repository.get_by_id(auth_id).await {
            Ok(info) => Ok(info),
            Err(_) => return Err(Status::InternalServerError)
        }
    }
    async fn get_circle_auth_info(&self, auth_id:&Uuid) -> Result<CircleAuthInfo, Status> {
        match self.circle_auth_info_repository.get_by_id(auth_id).await {
            Ok(info) => Ok(info),
            Err(_) => return Err(Status::InternalServerError)
        }
    }
    async  fn update_phase(&self, auth_id: &Uuid, phase: String) -> Result<usize, Status> {
        match self.auth_repository.update_phase(&auth_id, &phase).await {
            Ok(ok) => Ok(ok),
            Err(_) => return Err(Status::InternalServerError),
        }
    }
    async  fn delete(&self, auth_id: &Uuid) -> Result<usize, Status> {
        match self.locker_auth_info_repository.delete(&auth_id).await {
            Ok(_) => {},
            Err(_) => return Err(Status::InternalServerError),
        }
        match self.auth_repository.delete(&auth_id).await {
            Ok(ok) => Ok(ok),
            Err(_) => return Err(Status::InternalServerError),
        }
    }
}
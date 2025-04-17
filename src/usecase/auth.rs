use std::sync::Arc;
use std::env;
use crate::domain::{student::UserInfo, circle::OrganizationInfo};
use crate::adapters::repository::{RepositoryError, auth::AuthRepository, circle_auth_info::CircleAuthInfoRepository, locker_auth_info::LockerAuthInfoRepository};
use crate::infrastructure::models::{Auth, CircleAuthInfo, LockerAuthInfo};
use crate::utils::token::generate_token;

use dotenv::dotenv;
use uuid::Uuid;
use lettre::message::header::ContentType;
use lettre::{Message, SmtpTransport, Transport};
use rocket::{tokio::task, http::Status};
use async_trait::async_trait;

pub struct AuthUsecaseImpl {
     pub auth_repository: Arc<dyn AuthRepository>,
     pub locker_auth_info_repository: Arc<dyn LockerAuthInfoRepository>,
     pub circle_auth_info_repository: Arc<dyn CircleAuthInfoRepository>,
}

#[async_trait]
pub trait AuthUsecase: Sync + Send {
    async fn locker_register(&self, main_user: &UserInfo, co_user: &UserInfo, phase: &str, is_same: bool) -> Result<Auth, Status>;
    async fn circle_register(&self, organization: &OrganizationInfo, phase: &str, is_same: bool) -> Result<Auth, Status>;
    async fn mail_sender(&self, user_address: String, content: String, subject: &str) -> Result<(), Status>;
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
    async fn locker_register(&self, main_user: &UserInfo, co_user: &UserInfo, phase: &str, is_same: bool) -> Result<Auth, Status> {
        let main_token = generate_token();
        let mut co_token = generate_token();
        if is_same {
            co_token.clone_from(&main_token);
        }
        let phase = phase.to_string();
        let main_user = main_user.clone();
        let co_user = co_user.clone();
        let auth_repository = self.auth_repository.clone();
        let locker_auth_info_repository = self.locker_auth_info_repository.clone();

        let auth = match task::spawn_blocking(move || {
            auth_repository.insert(main_token, co_token, phase)
        }).await {
            Err(e) => {
                eprintln!("Thread panic in spawn_blocking: {:?}", e);
                return Err(Status::InternalServerError)
            },
            Ok(Err(RepositoryError::ConnectionError(e))) => {
                eprintln!("Connection Error: {:?}", e);
                return Err(Status::ServiceUnavailable)
            },
            Ok(Err(RepositoryError::DieselError(e))) => {
                eprintln!("Repository Error: {:?}", e);
                return Err(Status::InternalServerError)
            },
            Ok(Ok(auth)) => {auth},
        };

        match task::spawn_blocking(move || {
            locker_auth_info_repository.insert(auth.auth_id,
                                                main_user.student_id,
                                                main_user.family_name,
                                                main_user.given_name,
                                                co_user.student_id,
                                                co_user.family_name,
                                                co_user.given_name)
        }).await {
            Err(e) => {
                eprintln!("Thread panic in spawn_blocking: {:?}", e);
                return Err(Status::InternalServerError)
            },
            Ok(Err(RepositoryError::ConnectionError(e))) => {
                eprintln!("Connection Error: {:?}", e);
                return Err(Status::ServiceUnavailable)
            },
            Ok(Err(RepositoryError::DieselError(e))) => {
                eprintln!("Repository Error: {:?}", e);
                return Err(Status::InternalServerError)
            },
            Ok(_) => {return Ok(auth)},
        }
    }

    // 団体登録用、tokenの生成、DBへの登録
    async fn circle_register(&self, organization: &OrganizationInfo, phase: &str, is_same: bool) -> Result<Auth, Status> {
        let main_token = generate_token();
        let mut co_token = generate_token();
        if is_same {
            co_token.clone_from(&main_token);
        }
        let phase = phase.to_string();
        let organization = organization.clone();
        let auth_repository = self.auth_repository.clone();
        let circle_auth_info_repository = self.circle_auth_info_repository.clone();


        let auth = match task::spawn_blocking(move || {
            auth_repository.insert(main_token, co_token, phase)
        }).await {
            Err(e) => {
                eprintln!("Thread panic in spawn_blocking: {:?}", e);
                return Err(Status::InternalServerError)
            },
            Ok(Err(RepositoryError::ConnectionError(e))) => {
                eprintln!("Connection Error: {:?}", e);
                return Err(Status::ServiceUnavailable)
            },
            Ok(Err(RepositoryError::DieselError(e))) => {
                eprintln!("Repository Error: {:?}", e);
                return Err(Status::InternalServerError)
            },
            Ok(Ok(auth)) => {auth},
        };

        match task::spawn_blocking(move || {
            circle_auth_info_repository.insert(auth.auth_id,
                                                organization.main_user.student_id,
                                                organization.main_user.family_name,
                                                organization.main_user.given_name,
                                                organization.main_user.email,
                                                organization.main_user.phone_number,
                                                organization.co_user.student_id,
                                                organization.co_user.family_name,
                                                organization.co_user.given_name,
                                                organization.co_user.email,
                                                organization.co_user.phone_number,
                                                organization.b_doc,
                                                organization.c_doc,
                                                organization.d_doc,
                                                organization.organization.organization_name,
                                                organization.organization.organization_ruby,
                                                organization.organization.organization_email)
        }).await {
            Err(e) => {
                eprintln!("Thread panic in spawn_blocking: {:?}", e);
                return Err(Status::InternalServerError)
            },
            Ok(Err(RepositoryError::ConnectionError(e))) => {
                eprintln!("Connection Error: {:?}", e);
                return Err(Status::ServiceUnavailable)
            },
            Ok(Err(RepositoryError::DieselError(e))) => {
                eprintln!("Repository Error: {:?}", e);
                return Err(Status::InternalServerError)
            },
            Ok(_) => {return Ok(auth)},
        }
    }

    async fn mail_sender(&self, user_address: String, content: String, subject: &str) -> Result<(), Status> {
        // 環境変数の読み取り
        dotenv().ok();
        let sender_address = env::var("SENDER_MAIL_ADDRESS").expect("SENDER_MAIL_ADDRESS must be set.");
        let smtp_server = env::var("SMTP_SERVER").expect("SMTP_SERVER must be set.");

        let email = Message::builder()
            .from(
                format!("Noreply <{}>", sender_address)
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

        // SMTPサーバーに接続する
        let mailer = SmtpTransport::builder_dangerous(smtp_server.as_str())
            .port(25)
            .build();

        // メール送信
        mailer.send(&email).map_err(|_| Status::InternalServerError)?;

        Ok(())
    }

    async fn token_check(&self, token: String, is_main: bool) -> Result<Auth, Status> {
        let verify_token = token.clone();
        let repository = self.auth_repository.clone();

        let auth = match task::spawn_blocking(move || {
            repository.get_by_token(token)
        }).await {
            Err(e) => {
                eprintln!("Thread panic in spawn_blocking: {:?}", e);
                return Err(Status::InternalServerError)
            },
            Ok(Err(RepositoryError::ConnectionError(e))) => {
                eprintln!("Connection Error: {:?}", e);
                return Err(Status::ServiceUnavailable)
            },
            Ok(Err(RepositoryError::DieselError(diesel::result::Error::NotFound))) => {
                return Err(Status::InternalServerError)
            },
            Ok(Err(RepositoryError::DieselError(e))) => {
                eprintln!("Repository Error: {:?}", e);
                return Err(Status::Unauthorized)
            },
            Ok(Ok(auth)) => auth,
        };

        if (is_main && auth.main_auth_token == verify_token) || (!is_main && auth.co_auth_token == verify_token) {
            Ok(auth)
        } else {
            Err(Status::Unauthorized)
        }
    }

    async fn get_locker_auth_info(&self, auth_id: &Uuid) -> Result<LockerAuthInfo, Status> {
        let auth_id = *auth_id;
        let repository = self.locker_auth_info_repository.clone();

        match task::spawn_blocking(move || {
            repository.get_by_id(auth_id)
        }).await {
            Err(e) => {
                eprintln!("Thread panic in spawn_blocking: {:?}", e);
                return Err(Status::InternalServerError)
            },
            Ok(Err(RepositoryError::ConnectionError(e))) => {
                eprintln!("Connection Error: {:?}", e);
                return Err(Status::ServiceUnavailable)
            },
            Ok(Err(RepositoryError::DieselError(e))) => {
                eprintln!("Repository Error: {:?}", e);
                return Err(Status::InternalServerError)
            },
            Ok(Ok(info)) => Ok(info),
        }
    }

    async fn get_circle_auth_info(&self, auth_id:&Uuid) -> Result<CircleAuthInfo, Status> {
        let auth_id = *auth_id;
        let repository = self.circle_auth_info_repository.clone();

        match task::spawn_blocking(move || {
            repository.get_by_id(auth_id)
        }).await {
            Err(e) => {
                eprintln!("Thread panic in spawn_blocking: {:?}", e);
                return Err(Status::InternalServerError)
            },
            Ok(Err(RepositoryError::ConnectionError(e))) => {
                eprintln!("Connection Error: {:?}", e);
                return Err(Status::ServiceUnavailable)
            },
            Ok(Err(RepositoryError::DieselError(e))) => {
                eprintln!("Repository Error: {:?}", e);
                return Err(Status::InternalServerError)
            },
            Ok(Ok(info)) => Ok(info),
        }
    }

    async  fn update_phase(&self, auth_id: &Uuid, phase: String) -> Result<usize, Status> {
        let auth_id = *auth_id;
        let repository = self.auth_repository.clone();

        match task::spawn_blocking(move || {
            repository.update_phase(auth_id, phase)
        }).await {
            Err(e) => {
                eprintln!("Thread panic in spawn_blocking: {:?}", e);
                return Err(Status::InternalServerError)
            },
            Ok(Err(RepositoryError::ConnectionError(e))) => {
                eprintln!("Connection Error: {:?}", e);
                return Err(Status::ServiceUnavailable)
            },
            Ok(Err(RepositoryError::DieselError(e))) => {
                eprintln!("Repository Error: {:?}", e);
                return Err(Status::InternalServerError)
            },
            Ok(Ok(result)) => Ok(result),
        }
    }

    async  fn delete(&self, auth_id: &Uuid) -> Result<usize, Status> {
        let locker_auth_id = *auth_id;
        let circle_auth_id = *auth_id;
        let auth_id = *auth_id;
        let auth_repository = self.auth_repository.clone();
        let locker_auth_info_repository = self.locker_auth_info_repository.clone();
        let circle_auth_info_repository = self.circle_auth_info_repository.clone();

        match task::spawn_blocking(move || {
            locker_auth_info_repository.delete(locker_auth_id)
        }).await {
            Err(e) => {
                eprintln!("Thread panic in spawn_blocking: {:?}", e);
                return Err(Status::InternalServerError)
            },
            Ok(Err(RepositoryError::ConnectionError(e))) => {
                eprintln!("Connection Error: {:?}", e);
                return Err(Status::ServiceUnavailable)
            },
            Ok(Err(RepositoryError::DieselError(e))) => {
                eprintln!("Repository Error: {:?}", e);
                return Err(Status::InternalServerError)
            },
            Ok(_) => {},
        }

        match task::spawn_blocking(move || {
            circle_auth_info_repository.delete(circle_auth_id)
        }).await {
            Err(e) => {
                eprintln!("Thread panic in spawn_blocking: {:?}", e);
                return Err(Status::InternalServerError)
            },
            Ok(Err(RepositoryError::ConnectionError(e))) => {
                eprintln!("Connection Error: {:?}", e);
                return Err(Status::ServiceUnavailable)
            },
            Ok(Err(RepositoryError::DieselError(e))) => {
                eprintln!("Repository Error: {:?}", e);
                return Err(Status::InternalServerError)
            },
            Ok(_) => {},
        }

        match task::spawn_blocking(move || {
            auth_repository.delete(auth_id)
        }).await {
            Err(e) => {
                eprintln!("Thread panic in spawn_blocking: {:?}", e);
                return Err(Status::InternalServerError)
            },
            Ok(Err(RepositoryError::ConnectionError(e))) => {
                eprintln!("Connection Error: {:?}", e);
                return Err(Status::ServiceUnavailable)
            },
            Ok(Err(RepositoryError::DieselError(e))) => {
                eprintln!("Repository Error: {:?}", e);
                return Err(Status::InternalServerError)
            },
            Ok(Ok(result)) => Ok(result),
        }
    }
}
use std::sync::Arc;
use crate::domain::circle::Organization;
use crate::adapters::repository::{RepositoryError, organization::OrganizationRepository};
use crate::infrastructure::models;
use async_trait::async_trait;
use rocket::{tokio::task, http::Status};

pub struct OrganizationUsecaseImpl {
    pub organization_repository: Arc<dyn OrganizationRepository>,
}

#[async_trait]
pub trait OrganizationUsecase: Sync + Send {
    async fn register(&self, organization: &Organization) -> Result<models::Organization, Status>;
    async fn get_all(&self) -> Result<Vec<models::Organization>, Status>;
    async fn update_email(&self, organization_id: &i32, organization_email: &str) -> Result<models::Organization, Status>;
    async fn get_by_id(&self, organization_id: &i32) -> Result<models::Organization, Status>;
}

impl OrganizationUsecaseImpl {
    pub fn new(organization_repository: Arc<dyn OrganizationRepository>) -> Self {
        OrganizationUsecaseImpl { organization_repository }
    }
}

#[async_trait]
impl OrganizationUsecase for OrganizationUsecaseImpl {
    async fn register(&self, organization: &Organization) -> Result<models::Organization, Status> {
        let organization = organization.clone();
        let repository = self.organization_repository.clone();
        // 団体情報の登録
        match task::spawn_blocking(move || {
            repository.insert(organization.organization_name, organization.organization_ruby, organization.organization_email)
        }).await {
            Err(e) => {
                eprintln!("Thread panic in spawn_blocking: {:?}", e);
                Err(Status::InternalServerError)
            },
            Ok(Err(RepositoryError::ConnectionError(e))) => {
                eprintln!("Connection Error: {:?}", e);
                Err(Status::ServiceUnavailable)
            },
            Ok(Err(RepositoryError::DieselError(e))) => {
                eprintln!("Repository Error: {:?}", e);
                Err(Status::InternalServerError)
            },
            Ok(Ok(organization)) => Ok(organization),
        }
    }

    async fn get_all(&self) -> Result<Vec<models::Organization>, Status> {
        let repository = self.organization_repository.clone();

        match task::spawn_blocking(move || {
            repository.get_all()
        }).await {
            Err(e) => {
                eprintln!("Thread panic in spawn_blocking: {:?}", e);
                Err(Status::InternalServerError)
            },
            Ok(Err(RepositoryError::ConnectionError(e))) => {
                eprintln!("Connection Error: {:?}", e);
                Err(Status::ServiceUnavailable)
            },
            Ok(Err(RepositoryError::DieselError(e))) => {
                eprintln!("Repository Error: {:?}", e);
                Err(Status::InternalServerError)
            },
            Ok(Ok(organizations)) => Ok(organizations),
        }
    }

    async fn update_email(&self, organization_id: &i32, organization_email: &str) -> Result<models::Organization, Status> {
        let organization_id = *organization_id;
        let organization_email = organization_email.to_string();
        let repository = self.organization_repository.clone();

        // 団体メールアドレスの更新
        match task::spawn_blocking(move || {
            repository.update_email_by_id(organization_id, organization_email)
        }).await {
            Err(e) => {
                eprintln!("Thread panic in spawn_blocking: {:?}", e);
                Err(Status::InternalServerError)
            },
            Ok(Err(RepositoryError::ConnectionError(e))) => {
                eprintln!("Connection Error: {:?}", e);
                Err(Status::ServiceUnavailable)
            },
            Ok(Err(RepositoryError::DieselError(e))) => {
                eprintln!("Repository Error: {:?}", e);
                Err(Status::InternalServerError)
            },
            Ok(Ok(organization)) => Ok(organization),
        }
    }

    async fn get_by_id(&self, organization_id: &i32) -> Result<models::Organization, Status> {
        let organization_id = *organization_id;
        let repository = self.organization_repository.clone();

        // 団体情報の取得
        match task::spawn_blocking(move || {
            repository.get_by_id(organization_id)
        }).await {
            Err(e) => {
                eprintln!("Thread panic in spawn_blocking: {:?}", e);
                Err(Status::InternalServerError)
            },
            Ok(Err(RepositoryError::ConnectionError(e))) => {
                eprintln!("Connection Error: {:?}", e);
                Err(Status::ServiceUnavailable)
            },
            Ok(Err(RepositoryError::DieselError(e))) => {
                eprintln!("Repository Error: {:?}", e);
                Err(Status::InternalServerError)
            },
            Ok(Ok(organization)) => Ok(organization),
        }
    }
}
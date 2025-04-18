use std::sync::Arc;
use crate::adapters::repository::{RepositoryError, admin::AdminRepository};
use crate::infrastructure::models::Admin;
use async_trait::async_trait;
use rocket::{tokio::task, http::Status};

pub struct AdminUsecaseImpl {
    pub admin_repository: Arc<dyn AdminRepository>,
}

#[async_trait]
pub trait AdminUsecase: Sync + Send {
    async fn get_by_name<'a>(&self, username: &'a str) -> Result<Admin, Status>;
}

impl AdminUsecaseImpl {
    pub fn new(admin_repository: Arc<dyn AdminRepository>) -> Self {
        AdminUsecaseImpl { admin_repository }
    }
}

#[async_trait]
impl AdminUsecase for AdminUsecaseImpl {
    async fn get_by_name<'a>(&self, username: &'a str) -> Result<Admin, Status> {
        let username = username.to_string();
        let repository = self.admin_repository.clone();

        match task::spawn_blocking(move || {
            repository.get_by_name(username.to_string())
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
            Ok(Ok(admin)) => Ok(admin)
        }
    }
}
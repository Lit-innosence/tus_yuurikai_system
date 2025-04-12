use std::sync::Arc;
use crate::adapters::repository::admin::AdminRepository;
use crate::infrastructure::models::Admin;
use async_trait::async_trait;
use rocket::{tokio::task, http::Status};

pub struct AdminUsecaseImpl {
    pub admin_repository: Arc<dyn AdminRepository>,
}

#[async_trait]
pub trait AdminUsecase: Sync + Send {
    async fn get_by_name(&self, username: &String) -> Result<Admin, Status>;
}

impl AdminUsecaseImpl {
    pub fn new(admin_repository: Arc<dyn AdminRepository>) -> Self {
        AdminUsecaseImpl { admin_repository }
    }
}

#[async_trait]
impl AdminUsecase for AdminUsecaseImpl {
    async fn get_by_name(&self, username: &String) -> Result<Admin, Status> {
        let username = username.clone();
        let repository = self.admin_repository.clone();

        match task::spawn_blocking(move || {
            repository.get_by_name(username)
        }).await {
            Ok(Ok(admin)) => Ok(admin),
            _ => Err(Status::InternalServerError)
        }
    }
}
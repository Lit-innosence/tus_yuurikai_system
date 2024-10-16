use std::sync::Arc;
use crate::adapters::repository::AdminRepository;
use crate::infrastructure::models::Admin;
use diesel::result::Error;
use async_trait::async_trait;

pub struct AdminUsecaseImpl {
    pub admin_repository: Arc<dyn AdminRepository>,
}

#[async_trait]
pub trait AdminUsecase: Sync + Send {
    async fn get_by_name(&self, username: &String) -> Result<Admin, Error>;
}

impl AdminUsecaseImpl {
    pub fn new(admin_repository: Arc<dyn AdminRepository>) -> Self {
        AdminUsecaseImpl { admin_repository }
    }
}

#[async_trait]
impl AdminUsecase for AdminUsecaseImpl {
    async fn get_by_name(&self, username: &String) -> Result<Admin, Error> {
        self.admin_repository.get_by_name(username).await
    }
}
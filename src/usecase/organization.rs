use std::sync::Arc;
use crate::domain::circle::{Organization, OrganizationUpdateInfo, OrganizationInfo};
use crate::adapters::repository::OrganizationRepository;
use crate::infrastructure::models;
use diesel::result::Error;
use async_trait::async_trait;

pub struct OrganizationUsecaseImpl {
    pub organization_repository: Arc<dyn OrganizationRepository>,
}

#[async_trait]
pub trait OrganizationUsecase: Sync + Send {
    async fn register(&self, organization: &Organization) -> Result<models::Organization, Error>;

    // async fn update(&self, update_info: &OrganizationUpdateInfo) -> Result<Organization, Error>;
}

impl OrganizationUsecaseImpl {
    pub fn new(organization_repository: Arc<dyn OrganizationRepository>) -> Self {
        OrganizationUsecaseImpl { organization_repository }
    }
}

#[async_trait]
impl OrganizationUsecase for OrganizationUsecaseImpl {
    async fn register(&self, organization: &Organization) -> Result<models::Organization, Error> {
        // 団体情報の登録
        self.organization_repository.insert(&organization.organization_name, &organization.organization_ruby, &organization.organization_email).await
    }
}
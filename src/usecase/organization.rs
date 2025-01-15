use std::sync::Arc;
use crate::domain::circle::Organization;
use crate::adapters::repository::organization::OrganizationRepository;
use crate::infrastructure::models;
use diesel::result::Error;
use async_trait::async_trait;

pub struct OrganizationUsecaseImpl {
    pub organization_repository: Arc<dyn OrganizationRepository>,
}

#[async_trait]
pub trait OrganizationUsecase: Sync + Send {
    async fn register(&self, organization: &Organization) -> Result<models::Organization, Error>;

    async fn update_email(&self, organization_id: &i32, organization_email: &String) -> Result<models::Organization, Error>;

    async fn get_by_id(&self, organization_id: &i32) -> Result<models::Organization, Error>;
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

    async fn update_email(&self, organization_id: &i32, organization_email: &String) -> Result<models::Organization, Error> {
        // 団体メールアドレスの更新
        self.organization_repository.update_email_by_id(organization_id, organization_email).await
    }

    async fn get_by_id(&self, organization_id: &i32) -> Result<models::Organization, Error> {
        // 団体情報の取得
        self.organization_repository.get_by_id(organization_id).await
    }
}
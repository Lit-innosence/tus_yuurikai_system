use std::sync::Arc;
use crate::domain::circle::OrganizationInfo;
use crate::adapters::repository::RegistrationRepository;
use crate::infrastructure::models::Registration;
use diesel::result::Error;
use async_trait::async_trait;
use chrono::{Datelike, Local};


pub struct RegistrationUsecaseImpl {
    pub registration_repository: Arc<dyn RegistrationRepository>,
}

#[async_trait]
pub trait RegistrationUsecase: Sync + Send {
    async fn register(&self, organization: &OrganizationInfo, organization_id: &i32) -> Result<Registration, Error>;

    async fn update_student(&self, organization_id: &i32, main_student_id: &String, co_student_id: &String) -> Result<Registration, Error>;
}

impl RegistrationUsecaseImpl {
    pub fn new(registration_repository: Arc<dyn RegistrationRepository>) -> Self {
        RegistrationUsecaseImpl { registration_repository }
    }
}

#[async_trait]
impl RegistrationUsecase for RegistrationUsecaseImpl {
    async fn register(&self, organization: &OrganizationInfo, organization_id: &i32) -> Result<Registration, Error> {
        // 団体情報の登録
        let year = Local::now().year();
        let init_status = String::from("Pending");
        self.registration_repository.insert(organization_id,
                                            &year,
                                            &organization.main_user.student_id,
                                            &organization.co_user.student_id,
                                            &init_status,
                                            &init_status,
                                            &init_status,
                                            &init_status,
                                            &organization.b_doc,
                                            &organization.c_doc,
                                            &organization.d_doc
                                            ).await
    }

    async fn update_student(&self, organization_id: &i32, main_student_id: &String, co_student_id: &String) -> Result<Registration, Error> {
        // 団体代表者と団体副代表者の更新
        self.registration_repository.update_student_by_id(organization_id, main_student_id, co_student_id).await
    }
}
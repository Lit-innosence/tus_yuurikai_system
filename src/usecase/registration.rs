use std::sync::Arc;
use crate::domain::circle::OrganizationInfo;
use crate::adapters::repository::registration::RegistrationRepository;
use crate::infrastructure::models::Registration;
use async_trait::async_trait;
use rocket::{tokio::task, http::Status};
use chrono::{Datelike, Local};


pub struct RegistrationUsecaseImpl {
    pub registration_repository: Arc<dyn RegistrationRepository>,
}

#[async_trait]
pub trait RegistrationUsecase: Sync + Send {
    async fn register(&self, organization: &OrganizationInfo, organization_id: &i32) -> Result<Registration, Status>;
    async fn update_student(&self, organization_id: &i32, main_student_id: &String, co_student_id: &String) -> Result<Registration, Status>;
    async fn update_status(&self, organization_id: &i32, status_acceptance: &String, status_authentication: &String, status_form_confirmation: &String, status_registration_complete: &String) -> Result<Registration, Status>;
    async fn get_all(&self) -> Result<Vec<Registration>, Status>;
}

impl RegistrationUsecaseImpl {
    pub fn new(registration_repository: Arc<dyn RegistrationRepository>) -> Self {
        RegistrationUsecaseImpl { registration_repository }
    }
}

#[async_trait]
impl RegistrationUsecase for RegistrationUsecaseImpl {
    async fn register(&self, organization: &OrganizationInfo, organization_id: &i32) -> Result<Registration, Status> {
        // 団体情報の登録
        let year = Local::now().year();
        let init_status_acpt = String::from("pending");
        let init_status_auth = String::from("not_authenticated");
        let init_status_form = String::from("not_confirmed");
        let init_status_rgst = String::from("incomplete");
        let organization = organization.clone();
        let organization_id = *organization_id;
        let repository = self.registration_repository.clone();

        match task::spawn_blocking(move || {
            repository.insert(organization_id,
                            year,
                            organization.main_user.student_id,
                            organization.co_user.student_id,
                            init_status_acpt,
                            init_status_auth,
                            init_status_form,
                            init_status_rgst,
                            organization.b_doc,
                            organization.c_doc,
                            organization.d_doc
                            )
        }).await {
            Ok(Ok(registration)) => Ok(registration),
            _ => Err(Status::InternalServerError)
        }
    }

    async fn update_student(&self, organization_id: &i32, main_student_id: &String, co_student_id: &String) -> Result<Registration, Status> {
        let organization_id = *organization_id;
        let main_student_id = main_student_id.clone();
        let co_student_id = co_student_id.clone();
        let repository = self.registration_repository.clone();

        // 団体代表者と団体副代表者の更新
        match task::spawn_blocking(move || {
            repository.update_student_by_id(organization_id, main_student_id, co_student_id)
        }).await {
            Ok(Ok(registration)) => Ok(registration),
            _ => Err(Status::InternalServerError)
        }
    }

    async fn update_status(&self, organization_id: &i32, status_acceptance: &String, status_authentication: &String, status_form_confirmation: &String, status_registration_complete: &String) -> Result<Registration, Status> {
        let organization_id = *organization_id;
        let status_acceptance = status_acceptance.clone();
        let status_authentication = status_authentication.clone();
        let status_form_confirmation = status_form_confirmation.clone();
        let status_registration_complete = status_registration_complete.clone();
        let repository = self.registration_repository.clone();

        // 団体のステータス更新
        match task::spawn_blocking(move || {
            repository.update_status_by_id(organization_id, status_acceptance, status_authentication, status_form_confirmation, status_registration_complete)
        }).await {
            Ok(Ok(registration)) => Ok(registration),
            _ => Err(Status::InternalServerError)
        }
    }

    async fn get_all(&self) -> Result<Vec<Registration>, Status> {
        let repository = self.registration_repository.clone();

        // 団体情報を更新
        match task::spawn_blocking(move || {
            repository.get_all()
        }).await {
            Ok(Ok(registrations)) => Ok(registrations),
            _ => Err(Status::InternalServerError)
        }
    }
}
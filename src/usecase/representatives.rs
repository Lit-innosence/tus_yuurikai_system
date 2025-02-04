use std::sync::Arc;
use crate::adapters::repository::representatives::RepresentativesRepository;
use crate::domain::student::RepresentativeInfo;
use crate::infrastructure::models::Representatives;
use diesel::result::Error;
use async_trait::async_trait;

pub struct RepresentativesUsecaseImpl {
    pub representatives_repository: Arc<dyn RepresentativesRepository>,
}

#[async_trait]
pub trait RepresentativesUsecase: Sync + Send {
    async fn register(&self, student: &RepresentativeInfo) -> Result<Representatives, Error>;
    async fn get_by_id(&self, student_id: &String) -> Result<Representatives, Error>;
}

impl RepresentativesUsecaseImpl {
    pub fn new(representatives_repository: Arc<dyn RepresentativesRepository>) -> Self {
        RepresentativesUsecaseImpl { representatives_repository }
    }
}

#[async_trait]
impl RepresentativesUsecase for RepresentativesUsecaseImpl {
    async fn register(&self, student: &RepresentativeInfo) -> Result<Representatives, Error> {
        self.representatives_repository.insert(&student.student_id, &student.family_name, &student.given_name, &student.email, &student.phone_number).await
    }

    async fn get_by_id(&self, student_id: &String) -> Result<Representatives, Error> {
        self.representatives_repository.get_by_id(student_id).await
    }
}
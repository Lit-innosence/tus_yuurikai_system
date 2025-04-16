use std::sync::Arc;
use crate::adapters::repository::{RepositoryError, representatives::RepresentativesRepository};
use crate::domain::student::RepresentativeInfo;
use crate::infrastructure::models::Representatives;
use rocket::{tokio::task, http::Status};
use async_trait::async_trait;

pub struct RepresentativesUsecaseImpl {
    pub representatives_repository: Arc<dyn RepresentativesRepository>,
}

#[async_trait]
pub trait RepresentativesUsecase: Sync + Send {
    async fn register(&self, student: &RepresentativeInfo) -> Result<Representatives, Status>;
    async fn get_all(&self) -> Result<Vec<Representatives>, Status>;
    async fn get_by_id(&self, student_id: &str) -> Result<Representatives, Status>;
}

impl RepresentativesUsecaseImpl {
    pub fn new(representatives_repository: Arc<dyn RepresentativesRepository>) -> Self {
        RepresentativesUsecaseImpl { representatives_repository }
    }
}

#[async_trait]
impl RepresentativesUsecase for RepresentativesUsecaseImpl {
    async fn register(&self, student: &RepresentativeInfo) -> Result<Representatives, Status> {
        let student = student.clone();
        let repository = self.representatives_repository.clone();

        match task::spawn_blocking(move || {
            repository.insert(student.student_id, student.family_name, student.given_name, student.email, student.phone_number)
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
            Ok(Ok(representatives)) => Ok(representatives),
        }
    }

    async fn get_all(&self) -> Result<Vec<Representatives>, Status> {
        let repository = self.representatives_repository.clone();

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
            Ok(Ok(representativeses)) => Ok(representativeses),
        }
    }

    async fn get_by_id(&self, student_id: &str) -> Result<Representatives, Status> {
        let student_id = student_id.to_string();
        let repository = self.representatives_repository.clone();

        match task::spawn_blocking(move || {
            repository.get_by_id(student_id)
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
            Ok(Ok(representatives)) => Ok(representatives),
        }
    }
}
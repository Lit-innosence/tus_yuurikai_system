use std::sync::Arc;
use crate::domain::student::UserInfo;
use crate::adapters::repository::student::StudentRepository;
use crate::infrastructure::models::Student;
use rocket::{tokio::task, http::Status};
use async_trait::async_trait;

pub struct StudentUsecaseImpl {
    pub student_repository: Arc<dyn StudentRepository>,
}

#[async_trait]
pub trait StudentUsecase: Sync + Send {
    async fn register(&self, student: &UserInfo) -> Result<Student, Status>;
    async fn get_all(&self) -> Result<Vec<Student>, Status>;
    async fn get_by_id(&self, student_id: &String) -> Result<Student, Status>;
    async fn get_by_name(&self, family_name: &String, given_name: &String) -> Result<Vec<Student>, Status>;
}

impl StudentUsecaseImpl {
    pub fn new(student_repository: Arc<dyn StudentRepository>) -> Self {
        StudentUsecaseImpl { student_repository }
    }
}

#[async_trait]
impl StudentUsecase for StudentUsecaseImpl {
    async fn register(&self, student: &UserInfo) -> Result<Student, Status> {
        let student = student.clone();
        let repository = self.student_repository.clone();

        match task::spawn_blocking(move || {
            repository.insert(student.student_id, student.family_name, student.given_name)
        }).await {
            Ok(Ok(student)) => Ok(student),
            _ => Err(Status::InternalServerError)
        }
    }

    async fn get_all(&self) -> Result<Vec<Student>, Status> {
        let repository = self.student_repository.clone();

        match task::spawn_blocking(move || {
            repository.get_all()
        }).await {
            Ok(Ok(students)) => Ok(students),
            _ => Err(Status::InternalServerError)
        }
    }

    async  fn get_by_id(&self, student_id: &String) -> Result<Student, Status> {
        let student_id = student_id.clone();
        let repository = self.student_repository.clone();

        match task::spawn_blocking(move || {
            repository.get_by_id(student_id)
        }).await {
            Ok(Ok(student)) => Ok(student),
            _ => Err(Status::InternalServerError)
        }
    }

    async fn get_by_name(&self, family_name: &String, given_name: &String) -> Result<Vec<Student>, Status> {
        let family_name = family_name.clone();
        let given_name = given_name.clone();
        let repository = self.student_repository.clone();

        match task::spawn_blocking(move || {
            repository.get_by_name(family_name, given_name)
        }).await {
            Ok(Ok(students)) => Ok(students),
            _ => Err(Status::InternalServerError)
        }
    }
}
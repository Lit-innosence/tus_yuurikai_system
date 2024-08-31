use std::sync::Arc;
use crate::domain::student::UserInfo;
use crate::adapters::repository::StudentRepository;
use crate::infrastracture::models::Student;
use diesel::result::Error;
use async_trait::async_trait;

pub struct StudentUsecaseImpl {
    student_repository: Arc<dyn StudentRepository>,
}

#[async_trait]
pub trait StudentUsecase: Sync + Send {
    async fn register(&self, student: &UserInfo) -> Result<Student, Error>;
}

impl StudentUsecaseImpl {
    pub fn new(student_repository: Arc<dyn StudentRepository>) -> Self {
        StudentUsecaseImpl { student_repository }
    }
}

#[async_trait]
impl StudentUsecase for StudentUsecaseImpl {
    async fn register(&self, student: &UserInfo) -> Result<Student, Error> {
        self.student_repository.insert(&student.student_id, &student.family_name, &student.given_name).await
    }
}
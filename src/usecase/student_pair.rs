use std::sync::Arc;
use crate::domain::student_pair::PairInfo;
use crate::adapters::repository::student_pair::StudentPairRepository;
use crate::infrastructure::models::StudentPair;
use rocket::{tokio::task, http::Status};
use async_trait::async_trait;
use chrono::{Datelike, Local};

pub struct StudentPairUsecaseImpl {
    pub student_pair_repository: Arc<dyn StudentPairRepository>,
}

#[async_trait]
pub trait StudentPairUsecase: Sync + Send {
    async fn register(&self, student_pair: &PairInfo) -> Result<StudentPair, Status>;
    async fn get_all(&self) -> Result<Vec<StudentPair>, Status>;
    async fn get_by_id(&self, student_id: &String) -> Result<StudentPair, Status>;
    async fn get_by_main_id(&self, student_id: &String) -> Result<StudentPair, Status>;
    async fn get_by_pair_id(&self, pair_id: &uuid::Uuid) -> Result<StudentPair, Status>;
}

impl StudentPairUsecaseImpl {
    pub fn new(student_pair_repository: Arc<dyn StudentPairRepository>) -> Self {
        StudentPairUsecaseImpl { student_pair_repository }
    }
}

#[async_trait]
impl StudentPairUsecase for StudentPairUsecaseImpl {
    async fn register(&self, student_pair: &PairInfo) -> Result<StudentPair, Status> {
        let student_pair = student_pair.clone();
        let year = Local::now().year();
        let repository = self.student_pair_repository.clone();

        match task::spawn_blocking(move || {
            repository.insert(student_pair.main_user.student_id, student_pair.co_user.student_id, year)
        }).await {
            Ok(Ok(student_pair)) => Ok(student_pair),
            _ => Err(Status::InternalServerError)
        }
    }

    async fn get_all(&self) -> Result<Vec<StudentPair>, Status> {
        let repository = self.student_pair_repository.clone();

        match task::spawn_blocking(move || {
            repository.get_all()
        }).await {
            Ok(Ok(student_pairs)) => Ok(student_pairs),
            _ => Err(Status::InternalServerError)
        }
    }

    async fn get_by_id(&self, student_id: &String) -> Result<StudentPair, Status> {
        let student_id = student_id.clone();
        let year = Local::now().year();
        let repository = self.student_pair_repository.clone();

        match task::spawn_blocking(move || {
            repository.get_by_student_id_and_year(student_id, year)
        }).await {
            Ok(Ok(student_pair)) => Ok(student_pair),
            _ => Err(Status::InternalServerError)
        }
    }

    async fn get_by_main_id(&self, student_id: &String) -> Result<StudentPair, Status> {
        let student_id = student_id.clone();
        let year = Local::now().year();
        let repository = self.student_pair_repository.clone();

        match task::spawn_blocking(move || {
            repository.get_by_main_id_and_year(student_id, year)
        }).await {
            Ok(Ok(student_id)) => Ok(student_id),
            _ => Err(Status::InternalServerError)
        }
    }

    async fn get_by_pair_id(&self, pair_id: &uuid::Uuid) -> Result<StudentPair, Status> {
        let pair_id = pair_id.clone();
        let year = Local::now().year();
        let repository = self.student_pair_repository.clone();

        match task::spawn_blocking(move || {
            repository.get_by_pair_id_and_year(pair_id, year)
        }).await {
            Ok(Ok(student_pair)) => Ok(student_pair),
            _ => Err(Status::InternalServerError)
        }
    }
}
use std::sync::Arc;
use crate::domain::student_pair::PairInfo;
use crate::adapters::repository::StudentPairRepository;
use crate::infrastructure::models::StudentPair;
use diesel::result::Error;
use async_trait::async_trait;
use chrono::{Datelike, Local};

pub struct StudentPairUsecaseImpl {
    pub student_pair_repository: Arc<dyn StudentPairRepository>,
}

#[async_trait]
pub trait StudentPairUsecase: Sync + Send {
    async fn register(&self, student_pair: &PairInfo) -> Result<StudentPair, Error>;
    async fn get_by_id(&self, student_id: &String) -> Result<StudentPair, Error>;
    async fn get_by_main_id(&self, student_id: &String) -> Result<StudentPair, Error>;
    async fn get_by_pair_id(&self, pair_id: &uuid::Uuid) -> Result<StudentPair, Error>;
}

impl StudentPairUsecaseImpl {
    pub fn new(student_pair_repository: Arc<dyn StudentPairRepository>) -> Self {
        StudentPairUsecaseImpl { student_pair_repository }
    }
}

#[async_trait]
impl StudentPairUsecase for StudentPairUsecaseImpl {
    async fn register(&self, student_pair: &PairInfo) -> Result<StudentPair, Error> {
        let year = Local::now().year();
        self.student_pair_repository.insert(&student_pair.main_user.student_id, &student_pair.co_user.student_id, &year).await
    }
    async fn get_by_id(&self, student_id: &String) -> Result<StudentPair, Error> {
        let year = Local::now().year();
        self.student_pair_repository.get_by_student_id_and_year(student_id, &year).await
    }
    async fn get_by_main_id(&self, student_id: &String) -> Result<StudentPair, Error> {
        let year = Local::now().year();
        self.student_pair_repository.get_by_main_id_and_year(student_id, &year).await
    }
    async fn get_by_pair_id(&self, pair_id: &uuid::Uuid) -> Result<StudentPair, Error> {
        let year = Local::now().year();
        self.student_pair_repository.get_by_pair_id_and_year(pair_id, &year).await
    }
}
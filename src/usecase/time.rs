use std::sync::Arc;
use crate::adapters::repository::time::TimeRepository;
use crate::infrastructure::models::Time;
use chrono::NaiveDateTime;
use diesel::result::Error;
use async_trait::async_trait;

pub struct TimeUsecaseImpl {
    pub time_repository: Arc<dyn TimeRepository>,
}

#[async_trait]
pub trait TimeUsecase: Sync + Send {
    async fn register(&self, name: &String, start_time: &NaiveDateTime, end_time: &NaiveDateTime) -> Result<Time, Error>;
    async fn get_by_name(&self, name: &String) -> Result<Time, Error>;
}

impl TimeUsecaseImpl {
    pub fn new(time_repository: Arc<dyn TimeRepository>) -> Self {
        TimeUsecaseImpl { time_repository }
    }
}

#[async_trait]
impl TimeUsecase for TimeUsecaseImpl {
    async fn register(&self, name: &String, start_time: &NaiveDateTime, end_time: &NaiveDateTime) -> Result<Time, Error> {
        self.time_repository.insert(name, start_time, end_time).await
    }

    async fn get_by_name(&self, name: &String) -> Result<Time, Error> {
        self.time_repository.get_by_name(name).await
    }
}
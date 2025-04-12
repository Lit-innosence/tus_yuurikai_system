use std::sync::Arc;
use crate::adapters::repository::{self, time::TimeRepository};
use crate::infrastructure::models::Time;
use chrono::NaiveDateTime;
use rocket::{tokio::task, http::Status};
use async_trait::async_trait;

pub struct TimeUsecaseImpl {
    pub time_repository: Arc<dyn TimeRepository>,
}

#[async_trait]
pub trait TimeUsecase: Sync + Send {
    async fn register(&self, name: &String, start_time: &NaiveDateTime, end_time: &NaiveDateTime) -> Result<Time, Status>;
    async fn get_all(&self) -> Result<Vec<Time>, Status>;
    async fn get_by_name(&self, name: &String) -> Result<Time, Status>;
}

impl TimeUsecaseImpl {
    pub fn new(time_repository: Arc<dyn TimeRepository>) -> Self {
        TimeUsecaseImpl { time_repository }
    }
}

#[async_trait]
impl TimeUsecase for TimeUsecaseImpl {
    async fn register(&self, name: &String, start_time: &NaiveDateTime, end_time: &NaiveDateTime) -> Result<Time, Status> {
        let name = name.clone();
        let start_time = *start_time;
        let end_time = *end_time;
        let repository = self.time_repository.clone();

        match task::spawn_blocking(move || {
            repository.insert(name, start_time, end_time)
        }).await {
            Ok(Ok(time)) => Ok(time),
            _ => Err(Status::InternalServerError)
        }
    }

    async fn get_all(&self) -> Result<Vec<Time>, Status> {
        let repository = self.time_repository.clone();

        match task::spawn_blocking(move || {
            repository.get_all()
        }).await {
            Ok(Ok(times)) => Ok(times),
            _ => Err(Status::InternalServerError)
        }
    }

    async fn get_by_name(&self, name: &String) -> Result<Time, Status> {
        let name = name.clone();
        let repository = self.time_repository.clone();

        match task::spawn_blocking(move || {
            repository.get_by_name(name)
        }).await {
            Ok(Ok(time)) => Ok(time),
            _ => Err(Status::InternalServerError)
        }
    }
}
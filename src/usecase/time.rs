use std::sync::Arc;
use crate::adapters::repository::{RepositoryError, time::TimeRepository};
use crate::infrastructure::models::Time;
use chrono::NaiveDateTime;
use rocket::{tokio::task, http::Status};
use async_trait::async_trait;

pub struct TimeUsecaseImpl {
    pub time_repository: Arc<dyn TimeRepository>,
}

#[async_trait]
pub trait TimeUsecase: Sync + Send {
    async fn register(&self, name: &str, start_time: &NaiveDateTime, end_time: &NaiveDateTime) -> Result<Time, Status>;
    async fn get_all(&self) -> Result<Vec<Time>, Status>;
    async fn get_by_name(&self, name: &str) -> Result<Time, Status>;
}

impl TimeUsecaseImpl {
    pub fn new(time_repository: Arc<dyn TimeRepository>) -> Self {
        TimeUsecaseImpl { time_repository }
    }
}

#[async_trait]
impl TimeUsecase for TimeUsecaseImpl {
    async fn register(&self, name: &str, start_time: &NaiveDateTime, end_time: &NaiveDateTime) -> Result<Time, Status> {
        let name = name.to_string();
        let start_time = *start_time;
        let end_time = *end_time;
        let repository = self.time_repository.clone();

        match task::spawn_blocking(move || {
            repository.insert(name, start_time, end_time)
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
            Ok(Ok(time)) => Ok(time),
        }
    }

    async fn get_all(&self) -> Result<Vec<Time>, Status> {
        let repository = self.time_repository.clone();

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
            Ok(Ok(times)) => Ok(times),
        }
    }

    async fn get_by_name(&self, name: &str) -> Result<Time, Status> {
        let name = name.to_string();
        let repository = self.time_repository.clone();

        match task::spawn_blocking(move || {
            repository.get_by_name(name)
        }).await {
            Err(e) => {
                eprintln!("Thread panic in spawn_blocking: {:?}", e);
                Err(Status::InternalServerError)
            },
            Ok(Err(RepositoryError::ConnectionError(e))) => {
                eprintln!("Connection Error: {:?}", e);
                Err(Status::ServiceUnavailable)
            },
            Ok(Err(RepositoryError::DieselError(diesel::result::Error::NotFound))) => {
                Err(Status::InternalServerError)
            },
            Ok(Err(RepositoryError::DieselError(e))) => {
                eprintln!("Repository Error: {:?}", e);
                Err(Status::InternalServerError)
            },
            Ok(Ok(time)) => Ok(time),
        }
    }
}
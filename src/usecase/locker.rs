use std::sync::Arc;
use crate::adapters::repository::{RepositoryError, locker::LockerRepository};
use crate::infrastructure::models::Locker;
use async_trait::async_trait;
use rocket::{tokio::task, http::Status};

pub struct LockerUsecaseImpl {
    pub locker_repository: Arc<dyn LockerRepository>,
}

#[async_trait]
pub trait LockerUsecase: Sync + Send {
    async fn get_all(&self) -> Result<Vec<Locker>, Status>;
    async fn get_by_id(&self, locker_id: &str) -> Result<Locker, Status>;
    async fn get_by_floor(&self, floor: &Option<i8>) -> Result<Vec<Locker>, Status>;
    async fn update_status(&self, locker_id: &str, status: &str) -> Result<usize, Status>;
    async fn reset_status(&self) -> Result<usize, Status>;
}

impl LockerUsecaseImpl {
    pub fn new(locker_repository: Arc<dyn LockerRepository>) -> Self {
        LockerUsecaseImpl { locker_repository }
    }
}

#[async_trait]
impl LockerUsecase for LockerUsecaseImpl {
    async fn get_all(&self) -> Result<Vec<Locker>, Status> {
        let repository = self.locker_repository.clone();

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
            Ok(Ok(lockers)) => Ok(lockers),
        }
    }

    async fn get_by_id(&self, locker_id: &str) -> Result<Locker, Status> {
        let locker_id = locker_id.to_string();
        let repository = self.locker_repository.clone();

        match task::spawn_blocking(move || {
            repository.get_by_id(locker_id)
        }).await {
            Err(e) => {
                eprintln!("Thread panic in spawn_blocking: {:?}", e);
                return Err(Status::InternalServerError)
            },
            Ok(Err(RepositoryError::ConnectionError(e))) => {
                eprintln!("Connection Error: {:?}", e);
                return Err(Status::ServiceUnavailable)
            },
            Ok(Err(RepositoryError::DieselError(e))) => {
                eprintln!("Repository Error: {:?}", e);
                return Err(Status::InternalServerError)
            },
            Ok(Ok(locker)) => Ok(locker),
        }
    }

    async fn get_by_floor(&self, floor: &Option<i8>) -> Result<Vec<Locker>, Status> {
        let floor_val = match floor {
            None => String::from(""),
            Some(x) => format!("{}", x),
        };
        let repository = self.locker_repository.clone();

        match task::spawn_blocking(move || {
            repository.get_by_floor(floor_val)
        }).await {
            Err(e) => {
                eprintln!("Thread panic in spawn_blocking: {:?}", e);
                return Err(Status::InternalServerError)
            },
            Ok(Err(RepositoryError::ConnectionError(e))) => {
                eprintln!("Connection Error: {:?}", e);
                return Err(Status::ServiceUnavailable)
            },
            Ok(Err(RepositoryError::DieselError(e))) => {
                eprintln!("Repository Error: {:?}", e);
                return Err(Status::InternalServerError)
            },
            Ok(Ok(lockers)) => Ok(lockers),
        }
    }

    async  fn update_status(&self, locker_id: &str, status: &str) -> Result<usize, Status> {
        let locker_id = locker_id.to_string();
        let status = status.to_string();
        let repository = self.locker_repository.clone();

        match task::spawn_blocking(move || {
            repository.update_status_by_id(locker_id, status)
        }).await {
            Err(e) => {
                eprintln!("Thread panic in spawn_blocking: {:?}", e);
                return Err(Status::InternalServerError)
            },
            Ok(Err(RepositoryError::ConnectionError(e))) => {
                eprintln!("Connection Error: {:?}", e);
                return Err(Status::ServiceUnavailable)
            },
            Ok(Err(RepositoryError::DieselError(e))) => {
                eprintln!("Repository Error: {:?}", e);
                return Err(Status::InternalServerError)
            },
            Ok(Ok(result)) => Ok(result),
        }
    }

    async fn reset_status(&self) -> Result<usize, Status> {
        let repository = self.locker_repository.clone();

        // statusを更新
        match task::spawn_blocking(move || {
            repository.update_status(String::from(""), String::from("occupied"), String::from("vacant"))
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
            Ok(Ok(result)) => Ok(result),
        }
    }
}
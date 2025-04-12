use std::sync::Arc;
use crate::adapters::repository::locker::LockerRepository;
use crate::infrastructure::models::Locker;
use async_trait::async_trait;
use rocket::{tokio::task, http::Status};

pub struct LockerUsecaseImpl {
    pub locker_repository: Arc<dyn LockerRepository>,
}

#[async_trait]
pub trait LockerUsecase: Sync + Send {
    async fn get_all(&self) -> Result<Vec<Locker>, Status>;
    async fn get_by_id(&self, locker_id: &String) -> Result<Locker, Status>;
    async fn get_by_floor(&self, floor: &Option<i8>) -> Result<Vec<Locker>, Status>;
    async fn update_status(&self, locker_id: &String, status: &String) -> Result<usize, Status>;
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
            Ok(Ok(lockers)) => Ok(lockers),
            _ => Err(Status::InternalServerError)
        }
    }

    async fn get_by_id(&self, locker_id: &String) -> Result<Locker, Status> {
        let locker_id = locker_id.clone();
        let repository = self.locker_repository.clone();

        match task::spawn_blocking(move || {
            repository.get_by_id(locker_id)
        }).await {
            Ok(Ok(locker)) => Ok(locker),
            _ => Err(Status::InternalServerError)
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
            Ok(Ok(lockers)) => Ok(lockers),
            _ => Err(Status::InternalServerError)
        }
    }

    async  fn update_status(&self, locker_id: &String, status: &String) -> Result<usize, Status> {
        let locker_id = locker_id.clone();
        let status = status.clone();
        let repository = self.locker_repository.clone();

        match task::spawn_blocking(move || {
            repository.update_status_by_id(locker_id, status)
        }).await {
            Ok(Ok(result)) => Ok(result),
            _ => Err(Status::InternalServerError)
        }
    }

    async fn reset_status(&self) -> Result<usize, Status> {
        let repository = self.locker_repository.clone();

        // statusを更新
        match task::spawn_blocking(move || {
            repository.update_status(String::from(""), String::from("occupied"), String::from("vacant"))
        }).await {
            Ok(Ok(result)) => Ok(result),
            _ => Err(Status::InternalServerError),
        }
    }
}
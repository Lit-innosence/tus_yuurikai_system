use std::sync::Arc;
use crate::adapters::repository::LockerRepository;
use crate::infrastructure::models::Locker;
use diesel::result::Error;
use async_trait::async_trait;

pub struct LockerUsecaseImpl {
    pub locker_repository: Arc<dyn LockerRepository>,
}

#[async_trait]
pub trait LockerUsecase: Sync + Send {
    async fn get_by_id(&self, locker_id: &String) -> Result<Locker, Error>;
    async fn update_status(&self, locker_id: &String, status: &String) -> Result<usize, Error>;
}

impl LockerUsecaseImpl {
    pub fn new(locker_repository: Arc<dyn LockerRepository>) -> Self {
        LockerUsecaseImpl { locker_repository }
    }
}

#[async_trait]
impl LockerUsecase for LockerUsecaseImpl {
    async fn get_by_id(&self, locker_id: &String) -> Result<Locker, Error> {
        self.locker_repository.get_by_id(locker_id).await
    }

    async  fn update_status(&self, locker_id: &String, status: &String) -> Result<usize, Error> {
        self.locker_repository.update_status(locker_id, status).await
    }
}
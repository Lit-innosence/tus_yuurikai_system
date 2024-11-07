use std::sync::Arc;
use crate::adapters::repository::LockerRepository;
use crate::infrastructure::models::Locker;
use diesel::result::Error;
use async_trait::async_trait;
use rocket::http::Status;

pub struct LockerUsecaseImpl {
    pub locker_repository: Arc<dyn LockerRepository>,
}

#[async_trait]
pub trait LockerUsecase: Sync + Send {
    async fn get_by_id(&self, locker_id: &String) -> Result<Locker, Error>;
    async fn get_by_floor(&self, floor: &Option<i8>) -> Result<Vec<Locker>, Error>;
    async fn update_status(&self, locker_id: &String, status: &String) -> Result<usize, Error>;
    async fn reset_status(&self) -> Result<Status, Error>;
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

    async fn get_by_floor(&self, floor: &Option<i8>) -> Result<Vec<Locker>, Error> {
        let floor_val = match floor {
            None => String::from(""),
            Some(x) => format!("{}", x),
        };
        self.locker_repository.get_by_floor(&floor_val).await
    }

    async  fn update_status(&self, locker_id: &String, status: &String) -> Result<usize, Error> {
        self.locker_repository.update_status(locker_id, status).await
    }

    async fn reset_status(&self) -> Result<Status, Error> {

        // out-of-workのロッカーを取得
        let not_worked = match self.locker_repository.get_by_status(&String::from("out-of-work")).await {
            Ok(locker) => locker,
            Err(err) => {return Err(err)},
        };

        // statusを更新
        match self.locker_repository.update_all_status(&String::from("vacant")).await {
            Ok(_) => {},
            Err(err) => {return Err(err)},
        };

        // out-of-workのロッカーのstatusを修正
        for locker in not_worked {
            match self.locker_repository.update_status(&locker.locker_id, &locker.status).await {
                Ok(_) => {},
                Err(err) => {return Err(err)},
            }
        }

        Ok(Status::Ok)
    }
}
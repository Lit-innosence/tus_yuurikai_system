use std::sync::Arc;
use crate::domain::assignment::AssignmentInfo;
use crate::adapters::repository::assignment_record::AssignmentRecordRepository;
use crate::infrastructure::models::{AssignmentRecord, StudentPair};
use async_trait::async_trait;
use chrono::{Datelike, Local};
use rocket::http::Status;
use rocket::tokio::task;
use uuid::Uuid;

pub struct AssignmentRecordUsecaseImpl {
    pub assignment_record_repository: Arc<dyn AssignmentRecordRepository>,
}

#[async_trait]
pub trait AssignmentRecordUsecase: Sync + Send {
    async fn register(&self, student_pair: &StudentPair, assignment: &AssignmentInfo) -> Result<AssignmentRecord, Status>;
    async fn get_all(&self) -> Result<Vec<AssignmentRecord>, Status>;
    async fn get(&self, year: &i32, floor: Option<i8>, pair_id: &Uuid) -> Result<Vec<AssignmentRecord>, Status>;
    async fn get_by_pair_id(&self, pair_id: &Uuid) -> Result<Option<AssignmentRecord>, Status>;
}

impl AssignmentRecordUsecaseImpl {
    pub fn new(assignment_record_repository: Arc<dyn AssignmentRecordRepository>) -> Self {
        AssignmentRecordUsecaseImpl { assignment_record_repository }
    }
}

#[async_trait]
impl AssignmentRecordUsecase for AssignmentRecordUsecaseImpl {
    async fn register(&self, student_pair: &StudentPair, assignment: &AssignmentInfo) -> Result<AssignmentRecord, Status> {
        let pair_id = student_pair.pair_id.clone();
        let locker_id = assignment.locker_id.clone();
        let year = Local::now().year();
        let repository = self.assignment_record_repository.clone();

        match task::spawn_blocking(move || {
             repository.insert(pair_id, locker_id, year)
        }).await {
            Ok(Ok(assignment)) => Ok(assignment),
            _ => Err(Status::InternalServerError)
        }
    }

    async fn get_all(&self) -> Result<Vec<AssignmentRecord>, Status> {
        let repository = self.assignment_record_repository.clone();

        match task::spawn_blocking(move || {
            repository.get_all()
        }).await {
            Ok(Ok(result)) => Ok(result),
            _ => Err(Status::InternalServerError)
        }
    }

    async fn get(&self, year: &i32, floor: Option<i8>, pair_id: &Uuid) -> Result<Vec<AssignmentRecord>, Status> {
        let year = *year;
        let floor_val = match floor {
            None => String::from(""),
            Some(x) => format!("{}", x),
        };
        let pair_id = *pair_id;
        let repository = self.assignment_record_repository.clone();

        match task::spawn_blocking(move || {
            repository.get(year, floor_val, pair_id)
        }).await {
            Ok(Ok(result)) => Ok(result),
            _ => Err(Status::InternalServerError)
        }
    }

    async fn get_by_pair_id(&self, pair_id: &Uuid) -> Result<Option<AssignmentRecord>, Status> {
        let year = Local::now().year();
        let pair_id = *pair_id;
        let repository = self.assignment_record_repository.clone();

        match task::spawn_blocking(move || {
            repository.get_by_pair_id(year, pair_id)
        }).await {
            Ok(Ok(result)) => Ok(Some(result)),
            Ok(Err(diesel::NotFound)) => Ok(None),
            _ => Err(Status::InternalServerError)
        }
   }
}
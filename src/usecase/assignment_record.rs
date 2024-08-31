use std::sync::Arc;
use crate::domain::assignment::AssignmentInfo;
use crate::adapters::repository::AssignmentRecordRepository;
use crate::infrastracture::models::{StudentPair, AssignmentRecord};
use diesel::result::Error;
use async_trait::async_trait;
use chrono::{Datelike, Local};

pub struct AssignmentRecordUsecaseImpl {
    student_pair_repository: Arc<dyn AssignmentRecordRepository>,
}

#[async_trait]
pub trait AssignmentRecordUsecase: Sync + Send {
    async fn register(&self, student_pair: &StudentPair, assignment: &AssignmentInfo) -> Result<AssignmentRecord, Error>;
}

impl AssignmentRecordUsecaseImpl {
    pub fn new(student_pair_repository: Arc<dyn AssignmentRecordRepository>) -> Self {
        AssignmentRecordUsecaseImpl { student_pair_repository }
    }
}

#[async_trait]
impl AssignmentRecordUsecase for AssignmentRecordUsecaseImpl {
    async fn register(&self, student_pair: &StudentPair, assignment: &AssignmentInfo) -> Result<AssignmentRecord, Error> {
        let year = Local::now().year();
        self.student_pair_repository.insert(&student_pair.pair_id, &assignment.locker_id, &year).await
    }
}
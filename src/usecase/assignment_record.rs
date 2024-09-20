use std::sync::Arc;
use crate::domain::assignment::AssignmentInfo;
use crate::adapters::repository::AssignmentRecordRepository;
use crate::infrastracture::models::{StudentPair, AssignmentRecord};
use diesel::result::Error;
use async_trait::async_trait;
use chrono::{Datelike, Local};

pub struct AssignmentRecordUsecaseImpl {
    pub assignment_record_repository: Arc<dyn AssignmentRecordRepository>,
}

#[async_trait]
pub trait AssignmentRecordUsecase: Sync + Send {
    async fn register(&self, student_pair: &StudentPair, assignment: &AssignmentInfo) -> Result<AssignmentRecord, Error>;
}

impl AssignmentRecordUsecaseImpl {
    pub fn new(assignment_record_repository: Arc<dyn AssignmentRecordRepository>) -> Self {
        AssignmentRecordUsecaseImpl { assignment_record_repository }
    }
}

#[async_trait]
impl AssignmentRecordUsecase for AssignmentRecordUsecaseImpl {
    async fn register(&self, student_pair: &StudentPair, assignment: &AssignmentInfo) -> Result<AssignmentRecord, Error> {
        let year = Local::now().year();
        self.assignment_record_repository.insert(&student_pair.pair_id, &assignment.locker_id, &year).await
    }
}
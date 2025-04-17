use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid::Uuid;

use crate::infrastructure::schema::*;
use crate::infrastructure::models::*;
use crate::infrastructure::router::Pool;
use super::RepositoryError;

/// # assignment_record
pub trait AssignmentRecordRepository: Send + Sync {
    fn insert(
        &self,
        pair_id: Uuid,
        locker_id: String,
        year: i32,
    ) -> Result<AssignmentRecord, RepositoryError>;

    fn get_all(
        &self,
    ) -> Result<Vec<AssignmentRecord>, RepositoryError>;

    fn get(
        &self,
        year: i32,
        floor: String,
        pair_id: Uuid,
    ) -> Result<Vec<AssignmentRecord>, RepositoryError>;

    fn get_by_pair_id(
        &self,
        year: i32,
        pair_id: Uuid,
    ) -> Result<AssignmentRecord, RepositoryError>;

    fn delete_all(
        &self
    ) -> Result<usize, RepositoryError>;
}

pub struct AssignmentRecordRepositorySqlImpl {
    pool : Pool<PgConnection>
}

impl AssignmentRecordRepositorySqlImpl {
    pub fn new(pool: Pool<PgConnection>) -> Self {
        AssignmentRecordRepositorySqlImpl { pool }
    }
}

impl AssignmentRecordRepository for AssignmentRecordRepositorySqlImpl {
    fn insert (
        &self,
        pair_id: Uuid,
        locker_id: String,
        year: i32,
    ) -> Result<AssignmentRecord, RepositoryError> {
        let new_assignmentrecord = NewAssignmentRecord {
            pair_id: &pair_id,
            locker_id: &locker_id,
            year: &year
        };
        let mut conn = self.pool.get()?;
        let result = diesel::insert_into(assignment_record::table)
            .values(&new_assignmentrecord)
            .get_result::<AssignmentRecord>(&mut conn)?;

        Ok(result)
    }

    fn get_all(
        &self,
    ) -> Result<Vec<AssignmentRecord>, RepositoryError> {
        let mut conn = self.pool.get()?;
        let result = assignment_record::table
            .get_results::<AssignmentRecord>(&mut conn)?;

        Ok(result)
    }

    fn get (
        &self,
        year: i32,
        floor: String,
        pair_id: Uuid,
    ) -> Result<Vec<AssignmentRecord>, RepositoryError> {
        let mut conn = self.pool.get()?;

        let floor_ex = format!("{}%", floor);

        let result = assignment_record::table
            .filter(assignment_record::locker_id
                .like(floor_ex)
            ).filter(assignment_record::pair_id.eq(pair_id).and(assignment_record::year.eq(year))
            ).get_results::<AssignmentRecord>(&mut conn)?;

        Ok(result)
    }

    fn get_by_pair_id(
            &self,
            year: i32,
            pair_id: Uuid,
        ) -> Result<AssignmentRecord, RepositoryError> {
        let mut conn = self.pool.get()?;

        let result = assignment_record::table
            .filter(assignment_record::pair_id.eq(pair_id).and(assignment_record::year.eq(year)))
            .get_result::<AssignmentRecord>(&mut conn)?;

        Ok(result)
    }

    fn delete_all(
        &self
    ) -> Result<usize, RepositoryError> {
        let mut conn = self.pool.get()?;
        let result = diesel::delete(assignment_record::table)
            .execute(&mut conn)?;

        Ok(result)
    }
}
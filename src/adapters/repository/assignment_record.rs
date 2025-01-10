use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;
use async_trait::async_trait;

use crate::infrastructure::schema::*;
use crate::infrastructure::models::*;
use crate::infrastructure::router::Pool;

/// # assignment_record
#[async_trait]
pub trait AssignmentRecordRepository: Send + Sync {
    async fn insert(
        &self,
        pair_id: &Uuid,
        locker_id: &String,
        year: &i32,
    ) -> Result<AssignmentRecord, Error>;

    async fn get(
        &self,
        year: &i32,
        floor: &String,
        pair_id: &Uuid,
    ) -> Result<Vec<AssignmentRecord>, Error>;

    async fn get_by_pair_id(
        &self,
        year: &i32,
        pair_id: &Uuid,
    ) -> Result<AssignmentRecord, Error>;

    async fn delete_all(
        &self
    ) -> Result<usize, Error>;
}

pub struct AssignmentRecordRepositorySqlImpl {
    pool : Pool<PgConnection>
}

impl AssignmentRecordRepositorySqlImpl {
    pub fn new(pool: Pool<PgConnection>) -> Self {
        AssignmentRecordRepositorySqlImpl { pool }
    }
}

#[async_trait]
impl AssignmentRecordRepository for AssignmentRecordRepositorySqlImpl {
    async fn insert (
        &self,
        pair_id: &Uuid,
        locker_id: &String,
        year: &i32,
    ) -> Result<AssignmentRecord, Error> {
        let new_assignmentrecord = NewAssignmentRecord {
            pair_id,
            locker_id,
            year,
        };
        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(assignment_record::table)
            .values(&new_assignmentrecord)
            .get_result(&mut conn)
    }

    async fn get (
        &self,
        year: &i32,
        floor: &String,
        pair_id: &Uuid,
    ) -> Result<Vec<AssignmentRecord>, Error> {
        let mut conn = self.pool.get().unwrap();

        let floor_ex = format!("{}%", floor);

        assignment_record::table
        .filter(assignment_record::locker_id
            .like(floor_ex)
        ).filter(assignment_record::pair_id.eq(pair_id).and(assignment_record::year.eq(year))
        ).get_results(&mut conn)
    }

    async  fn get_by_pair_id(
            &self,
            year: &i32,
            pair_id: &Uuid,
        ) -> Result<AssignmentRecord, Error> {
        let mut conn = self.pool.get().unwrap();

        assignment_record::table
            .filter(assignment_record::pair_id.eq(pair_id).and(assignment_record::year.eq(year)))
            .get_result(&mut conn)
    }

    async fn delete_all(
        &self
    ) -> Result<usize, Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::delete(assignment_record::table)
            .execute(&mut conn)
    }
}
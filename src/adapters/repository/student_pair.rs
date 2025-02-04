use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;
use async_trait::async_trait;

use crate::infrastructure::schema::*;
use crate::infrastructure::models::*;
use crate::infrastructure::router::Pool;

/// # student_pair
#[async_trait]
pub trait StudentPairRepository: Send + Sync {
    async fn insert(
        &self,
        student_id1: &String,
        student_id2: &String,
        year: &i32,
    ) -> Result<StudentPair, Error>;
    async fn get_by_student_id_and_year(
        &self,
        student_id: &String,
        year: &i32,
    ) -> Result<StudentPair, Error>;
    async fn get_by_main_id_and_year(
        &self,
        student_id: &String,
        year: &i32,
    ) -> Result<StudentPair, Error>;
    async fn get_by_pair_id_and_year(
        &self,
        pair_id : &Uuid,
        year: &i32,
    ) -> Result<StudentPair, Error>;
    async fn delete_all(
        &self
    ) -> Result<usize, Error>;
}

pub struct StudentPairRepositorySqlImpl{
    pool: Pool<PgConnection>
}

impl StudentPairRepositorySqlImpl {
    pub fn new(pool: Pool<PgConnection>) -> Self {
        StudentPairRepositorySqlImpl { pool }
    }
}

#[async_trait]
impl StudentPairRepository for StudentPairRepositorySqlImpl{
    async fn insert(
        &self,
        student_id1: &String,
        student_id2: &String,
        year: &i32,
    ) -> Result<StudentPair, Error> {
        let new_studentpair = NewStudentPair {
            student_id1,
            student_id2,
            year,
        };
        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(student_pair::table)
            .values(&new_studentpair)
            .get_result(&mut conn)
    }

    async fn get_by_student_id_and_year(
        &self,
        student_id: &String,
        year: &i32,
    ) -> Result<StudentPair, Error> {
        let mut conn = self.pool.get().unwrap();
        student_pair::table
            .filter(
                student_pair::student_id1
                    .eq(student_id)
                    .or(student_pair::student_id2.eq(student_id))
                    .and(student_pair::year.eq(year))
            )
            .first(&mut conn)
    }

    async fn get_by_main_id_and_year(
        &self,
        student_id: &String,
        year: &i32,
    ) -> Result<StudentPair, Error> {
        let mut conn = self.pool.get().unwrap();
        student_pair::table
            .filter(
                student_pair::student_id1
                    .eq(student_id)
                    .and(student_pair::year.eq(year))
            )
            .first(&mut conn)
    }

    async fn get_by_pair_id_and_year(
        &self,
        pair_id: &Uuid,
        year: &i32,
    ) -> Result<StudentPair, Error> {
        let mut conn = self.pool.get().unwrap();
        student_pair::table
            .filter(
                student_pair::pair_id
                    .eq(pair_id)
                    .and(student_pair::year.eq(year))
            ).get_result(&mut conn)
    }

    async fn delete_all(
        &self
    ) -> Result<usize, Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::delete(student_pair::table)
            .execute(&mut conn)
    }
}
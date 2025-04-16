use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid::Uuid;

use crate::infrastructure::schema::*;
use crate::infrastructure::models::*;
use crate::infrastructure::router::Pool;
use super::RepositoryError;

/// # student_pair
pub trait StudentPairRepository: Send + Sync {
    fn insert(
        &self,
        student_id1: String,
        student_id2: String,
        year: i32,
    ) -> Result<StudentPair, RepositoryError>;

    fn get_all(
        &self,
    ) -> Result<Vec<StudentPair>, RepositoryError>;

    fn get_by_student_id_and_year(
        &self,
        student_id: String,
        year: i32,
    ) -> Result<StudentPair, RepositoryError>;

    fn get_by_main_id_and_year(
        &self,
        student_id: String,
        year: i32,
    ) -> Result<StudentPair, RepositoryError>;

    fn get_by_pair_id_and_year(
        &self,
        pair_id : Uuid,
        year: i32,
    ) -> Result<StudentPair, RepositoryError>;

    fn delete_all(
        &self
    ) -> Result<usize, RepositoryError>;
}

pub struct StudentPairRepositorySqlImpl{
    pool: Pool<PgConnection>
}

impl StudentPairRepositorySqlImpl {
    pub fn new(pool: Pool<PgConnection>) -> Self {
        StudentPairRepositorySqlImpl { pool }
    }
}

impl StudentPairRepository for StudentPairRepositorySqlImpl{
    fn insert(
        &self,
        student_id1: String,
        student_id2: String,
        year: i32,
    ) -> Result<StudentPair, RepositoryError> {
        let new_studentpair = NewStudentPair {
            student_id1: &student_id1,
            student_id2: &student_id2,
            year: &year,
        };
        let mut conn = self.pool.get()?;
        let result = diesel::insert_into(student_pair::table)
            .values(&new_studentpair)
            .get_result::<StudentPair>(&mut conn)?;

        Ok(result)
    }

    fn get_all(
        &self,
    ) -> Result<Vec<StudentPair>, RepositoryError> {
        let mut conn = self.pool.get()?;
        let result = student_pair::table
            .get_results::<StudentPair>(&mut conn)?;

        Ok(result)
    }

    fn get_by_student_id_and_year(
        &self,
        student_id: String,
        year: i32,
    ) -> Result<StudentPair, RepositoryError> {
        let mut conn = self.pool.get()?;
        let result = student_pair::table
            .filter(
                student_pair::student_id1
                    .eq(student_id.clone())
                    .or(student_pair::student_id2.eq(student_id))
                    .and(student_pair::year.eq(year))
            )
            .first::<StudentPair>(&mut conn)?;

        Ok(result)
    }

    fn get_by_main_id_and_year(
        &self,
        student_id: String,
        year: i32,
    ) -> Result<StudentPair, RepositoryError> {
        let mut conn = self.pool.get()?;
        let result = student_pair::table
            .filter(
                student_pair::student_id1
                    .eq(student_id)
                    .and(student_pair::year.eq(year))
            )
            .first::<StudentPair>(&mut conn)?;

        Ok(result)
    }

    fn get_by_pair_id_and_year(
        &self,
        pair_id: Uuid,
        year: i32,
    ) -> Result<StudentPair, RepositoryError> {
        let mut conn = self.pool.get()?;
        let result = student_pair::table
            .filter(
                student_pair::pair_id
                    .eq(pair_id)
                    .and(student_pair::year.eq(year))
            ).get_result::<StudentPair>(&mut conn)?;

        Ok(result)
    }

    fn delete_all(
        &self
    ) -> Result<usize, RepositoryError> {
        let mut conn = self.pool.get()?;
        let result = diesel::delete(student_pair::table)
            .execute(&mut conn)?;

        Ok(result)
    }
}
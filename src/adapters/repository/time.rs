use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::infrastructure::schema::*;
use crate::infrastructure::models::*;
use crate::infrastructure::router::Pool;
use super::RepositoryError;

/// # time
pub trait TimeRepository: Send + Sync {
    fn insert(
        &self,
        name: String,
        start_time: NaiveDateTime,
        end_time: NaiveDateTime,
    ) -> Result<Time, RepositoryError>;

    fn get_all(
        &self,
    ) -> Result<Vec<Time>, RepositoryError>;

    fn get_by_name(
        &self,
        name: String,
    ) -> Result<Time, RepositoryError>;
}

pub struct TimeRepositorySqlImpl {
    pool: Pool<PgConnection>
}

impl TimeRepositorySqlImpl {
    pub fn new(pool: Pool<PgConnection>) -> Self {
        TimeRepositorySqlImpl { pool }
    }
}

impl TimeRepository for TimeRepositorySqlImpl {
    fn insert(
            &self,
            name: String,
            start_time: NaiveDateTime,
            end_time: NaiveDateTime,
        ) -> Result<Time, RepositoryError> {
        let new_time = NewTime{
            name: &name,
            start_time: &start_time,
            end_time: &end_time,
        };
        let mut conn = self.pool.get()?;
        let result = diesel::insert_into(time::table)
            .values(&new_time)
            .on_conflict(time::name)
            .do_update()
            .set((time::start_time.eq(start_time), time::end_time.eq(end_time), time::updated_at.eq(diesel::dsl::now)))
            .get_result::<Time>(&mut conn)?;

        Ok(result)
    }

    fn get_all(
            &self,
        ) -> Result<Vec<Time>, RepositoryError> {
        let mut conn = self.pool.get()?;
        let result = time::table
            .get_results::<Time>(&mut conn)?;

        Ok(result)
    }

    fn get_by_name(
            &self,
            name: String,
        ) -> Result<Time, RepositoryError> {
        let mut conn = self.pool.get()?;
        let result = time::table
            .filter(time::name.eq(name))
            .get_result::<Time>(&mut conn)?;

        Ok(result)
    }
}
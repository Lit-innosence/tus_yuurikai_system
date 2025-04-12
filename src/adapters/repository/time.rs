use chrono::NaiveDateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

use crate::infrastructure::schema::*;
use crate::infrastructure::models::*;
use crate::infrastructure::router::Pool;

/// # time
pub trait TimeRepository: Send + Sync {
    fn insert(
        &self,
        name: String,
        start_time: NaiveDateTime,
        end_time: NaiveDateTime,
    ) -> Result<Time, Error>;

    fn get_all(
        &self,
    ) -> Result<Vec<Time>, Error>;

    fn get_by_name(
        &self,
        name: String,
    ) -> Result<Time, Error>;
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
        ) -> Result<Time, Error> {
        let new_time = NewTime{
            name: &name,
            start_time: &start_time,
            end_time: &end_time,
        };
        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(time::table)
            .values(&new_time)
            .on_conflict(time::name)
            .do_update()
            .set((time::start_time.eq(start_time), time::end_time.eq(end_time), time::updated_at.eq(diesel::dsl::now)))
            .get_result(&mut conn)
    }

    fn get_all(
            &self,
        ) -> Result<Vec<Time>, Error> {
        let mut conn = self.pool.get().unwrap();
        time::table
            .get_results(&mut conn)
    }

    fn get_by_name(
            &self,
            name: String,
        ) -> Result<Time, Error> {
        let mut conn = self.pool.get().unwrap();
        time::table
            .filter(time::name.eq(name))
            .get_result(&mut conn)
    }
}
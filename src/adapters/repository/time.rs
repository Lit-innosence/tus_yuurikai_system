use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use async_trait::async_trait;

use crate::infrastructure::schema::*;
use crate::infrastructure::models::*;
use crate::infrastructure::router::Pool;

/// # time
#[async_trait]
pub trait TimeRepository: Send + Sync {
    async fn insert(
        &self,
        name: &String,
        start_time: &String,
        end_time: &String,
    ) -> Result<Time, Error>;

    async fn get_by_name(
        &self,
        name: &String,
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

#[async_trait]
impl TimeRepository for TimeRepositorySqlImpl {
    async fn insert(
            &self,
            name: &String,
            start_time: &String,
            end_time: &String,
        ) -> Result<Time, Error> {
        let new_time = NewTime{
            name: name,
            start_time: start_time,
            end_time: end_time,
        };
        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(time::table)
            .values(&new_time)
            .on_conflict(time::name)
            .do_update()
            .set((time::start_time.eq(start_time), time::end_time.eq(end_time), time::updated_at.eq(diesel::dsl::now)))
            .get_result(&mut conn)
    }
    async fn get_by_name(
            &self,
            name: &String,
        ) -> Result<Time, Error> {
        let mut conn = self.pool.get().unwrap();
        time::table
            .filter(time::name.eq(name))
            .get_result(&mut conn)
    }
}
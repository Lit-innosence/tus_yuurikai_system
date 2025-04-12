use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

use crate::infrastructure::schema::*;
use crate::infrastructure::models::*;
use crate::infrastructure::router::Pool;

/// # locker
pub trait LockerRepository: Send + Sync {
    fn insert(
        &self,
        locker_id: String,
        location: String,
        status: String,
    ) -> Result<Locker, Error>;

    fn get_all(
        &self,
    ) -> Result<Vec<Locker>, Error>;

    fn update_status(
        &self,
        floor: String,
        prev_status: String,
        new_status: String,
    ) -> Result<usize, Error>;

    fn update_status_by_id(
        &self,
        locker_id: String,
        status: String,
    ) -> Result<usize, Error>;

    fn update_all_status(
        &self,
        status: String,
    ) -> Result<usize, Error>;

    fn get_by_id(
        &self,
        locker_id: String,
    ) -> Result<Locker, Error>;

    fn get_by_floor(
        &self,
        floor: String,
    ) -> Result<Vec<Locker>, Error>;

    fn get_by_status(
        &self,
        status: String,
    ) -> Result<Vec<Locker>, Error>;

    fn delete_all(
        &self
    ) -> Result<usize, Error>;
}

pub struct LockerRepositorySqlImpl {
    pool: Pool<PgConnection>
}

impl LockerRepositorySqlImpl {
    pub fn new(pool: Pool<PgConnection>) -> Self {
        LockerRepositorySqlImpl { pool }
    }
}

impl LockerRepository for LockerRepositorySqlImpl {
    fn insert(
        &self,
        locker_id: String,
        location: String,
        status: String,
    ) -> Result<Locker, Error> {
        let new_locker = NewLocker {
            locker_id: &locker_id,
            location: &location,
            status: &status
        };
        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(locker::table)
            .values(&new_locker)
            .get_result(&mut conn)
    }

    fn get_all(
        &self,
    ) -> Result<Vec<Locker>, Error> {
        let mut conn = self.pool.get().unwrap();
        locker::table
            .get_results(&mut conn)
    }

    fn update_status(
            &self,
            floor: String,
            prev_status: String,
            new_status: String,
        ) -> Result<usize, Error> {
        let mut conn = self.pool.get().unwrap();
        let floor_ex = format!("{}%", floor);
        let status_ex = format!("{}%", prev_status);
        diesel::update(
            locker::table.filter(
                locker::locker_id.like(floor_ex)
            ).filter(locker::status.like(status_ex))
        ).set(locker::status.eq(new_status))
        .execute(&mut conn)
    }

    fn update_status_by_id(
            &self,
            locker_id: String,
            status: String,
        ) -> Result<usize, Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::update(locker::table.find(locker_id))
            .set(locker::status.eq(status))
            .execute(&mut conn)
    }

    fn update_all_status(
            &self,
            status: String,
        ) -> Result<usize, Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::update(locker::table)
            .set(locker::status.eq(status))
            .execute(&mut conn)
    }

    fn get_by_id(
            &self,
            locker_id: String,
        ) -> Result<Locker, Error> {
            let mut conn = self.pool.get().unwrap();
        locker::table
            .filter(
                locker::locker_id
                .eq(locker_id)
            ).first(&mut conn)
    }

    fn get_by_floor(
            &self,
            floor: String,
        ) -> Result<Vec<Locker>, Error> {
        let mut conn = self.pool.get().unwrap();
        let floor_ex = format!("{}%", floor);
        locker::table
            .filter(
                locker::locker_id
                .like(floor_ex)
            ).get_results(&mut conn)
    }

    fn get_by_status(
            &self,
            status: String,
        ) -> Result<Vec<Locker>, Error> {
        let mut conn = self.pool.get().unwrap();
        locker::table
            .filter(
                locker::status.like(status)
            ).get_results(&mut conn)
    }

    fn delete_all(
        &self
    ) -> Result<usize, Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::delete(locker::table)
            .execute(&mut conn)
    }
}
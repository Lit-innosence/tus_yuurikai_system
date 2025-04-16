use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::infrastructure::schema::*;
use crate::infrastructure::models::*;
use crate::infrastructure::router::Pool;
use super::RepositoryError;

/// # locker
pub trait LockerRepository: Send + Sync {
    fn insert(
        &self,
        locker_id: String,
        location: String,
        status: String,
    ) -> Result<Locker, RepositoryError>;

    fn get_all(
        &self,
    ) -> Result<Vec<Locker>, RepositoryError>;

    fn update_status(
        &self,
        floor: String,
        prev_status: String,
        new_status: String,
    ) -> Result<usize, RepositoryError>;

    fn update_status_by_id(
        &self,
        locker_id: String,
        status: String,
    ) -> Result<usize, RepositoryError>;

    fn update_all_status(
        &self,
        status: String,
    ) -> Result<usize, RepositoryError>;

    fn get_by_id(
        &self,
        locker_id: String,
    ) -> Result<Locker, RepositoryError>;

    fn get_by_floor(
        &self,
        floor: String,
    ) -> Result<Vec<Locker>, RepositoryError>;

    fn get_by_status(
        &self,
        status: String,
    ) -> Result<Vec<Locker>, RepositoryError>;

    fn delete_all(
        &self
    ) -> Result<usize, RepositoryError>;
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
    ) -> Result<Locker, RepositoryError> {
        let new_locker = NewLocker {
            locker_id: &locker_id,
            location: &location,
            status: &status
        };
        let mut conn = self.pool.get()?;
        let result = diesel::insert_into(locker::table)
            .values(&new_locker)
            .get_result::<Locker>(&mut conn)?;

        Ok(result)
    }

    fn get_all(
        &self,
    ) -> Result<Vec<Locker>, RepositoryError> {
        let mut conn = self.pool.get()?;
        let result = locker::table
            .get_results::<Locker>(&mut conn)?;

        Ok(result)
    }

    fn update_status(
            &self,
            floor: String,
            prev_status: String,
            new_status: String,
        ) -> Result<usize, RepositoryError> {
        let mut conn = self.pool.get()?;
        let floor_ex = format!("{}%", floor);
        let status_ex = format!("{}%", prev_status);
        let result = diesel::update(
            locker::table.filter(
                locker::locker_id.like(floor_ex)
            ).filter(locker::status.like(status_ex))
            ).set(locker::status.eq(new_status))
            .execute(&mut conn)?;

        Ok(result)
    }

    fn update_status_by_id(
            &self,
            locker_id: String,
            status: String,
        ) -> Result<usize, RepositoryError> {
        let mut conn = self.pool.get()?;
        let result = diesel::update(locker::table.find(locker_id))
            .set(locker::status.eq(status))
            .execute(&mut conn)?;

        Ok(result)
    }

    fn update_all_status(
            &self,
            status: String,
        ) -> Result<usize, RepositoryError> {
        let mut conn = self.pool.get()?;
        let result = diesel::update(locker::table)
            .set(locker::status.eq(status))
            .execute(&mut conn)?;

        Ok(result)
    }

    fn get_by_id(
            &self,
            locker_id: String,
        ) -> Result<Locker, RepositoryError> {
            let mut conn = self.pool.get()?;
        let result = locker::table
            .filter(
                locker::locker_id
                .eq(locker_id)
            ).first::<Locker>(&mut conn)?;

        Ok(result)
    }

    fn get_by_floor(
            &self,
            floor: String,
        ) -> Result<Vec<Locker>, RepositoryError> {
        let mut conn = self.pool.get()?;
        let floor_ex = format!("{}%", floor);
        let result = locker::table
            .filter(
                locker::locker_id
                .like(floor_ex)
            ).get_results(&mut conn)?;

        Ok(result)
    }

    fn get_by_status(
            &self,
            status: String,
        ) -> Result<Vec<Locker>, RepositoryError> {
        let mut conn = self.pool.get()?;
        let result = locker::table
            .filter(
                locker::status.like(status)
            ).get_results::<Locker>(&mut conn)?;

        Ok(result)
    }

    fn delete_all(
        &self
    ) -> Result<usize, RepositoryError> {
        let mut conn = self.pool.get()?;
        let result = diesel::delete(locker::table)
            .execute(&mut conn)?;

        Ok(result)
    }
}
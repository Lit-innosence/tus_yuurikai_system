use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid::Uuid;

use crate::infrastructure::schema::*;
use crate::infrastructure::models::*;
use crate::infrastructure::router::Pool;
use super::RepositoryError;

/// # locker_auth_info
pub trait LockerAuthInfoRepository: Send + Sync {
    fn insert(
        &self,
        auth_id: Uuid,
        main_student_id: String,
        main_family_name: String,
        main_given_name: String,
        co_student_id: String,
        co_family_name: String,
        co_given_name: String,
    ) -> Result<LockerAuthInfo, RepositoryError>;

    fn get_by_id(
        &self,
        auth_id: Uuid,
    ) -> Result<LockerAuthInfo, RepositoryError>;

    fn delete(
        &self,
        auth_id: Uuid,
    ) -> Result<usize, RepositoryError>;

    fn delete_all(
        &self
    ) -> Result<usize, RepositoryError>;
}

pub struct LockerAuthInfoRepositorySqlImpl {
    pool: Pool<PgConnection>
}

impl LockerAuthInfoRepositorySqlImpl {
    pub fn new(pool: Pool<PgConnection>) -> Self {
        LockerAuthInfoRepositorySqlImpl { pool }
    }
}

impl LockerAuthInfoRepository for LockerAuthInfoRepositorySqlImpl {
    fn insert(
            &self,
            auth_id: Uuid,
            main_student_id: String,
            main_family_name: String,
            main_given_name: String,
            co_student_id: String,
            co_family_name: String,
            co_given_name: String,
    ) -> Result<LockerAuthInfo, RepositoryError> {
        let new_auth_info = NewLockerAuthInfo {
            auth_id: &auth_id,
            main_student_id: &main_student_id,
            main_family_name: &main_family_name,
            main_given_name: &main_given_name,
            co_student_id: &co_student_id,
            co_family_name: &co_family_name,
            co_given_name: &co_given_name,
        };
        let mut conn = self.pool.get()?;
        let result = diesel::insert_into(locker_auth_info::table)
            .values(&new_auth_info)
            .get_result::<LockerAuthInfo>(&mut conn)?;

        Ok(result)
    }

    fn get_by_id(
            &self,
            auth_id: Uuid,
        ) -> Result<LockerAuthInfo, RepositoryError> {
            let mut conn = self.pool.get()?;
        let result = locker_auth_info::table.filter(locker_auth_info::auth_id.eq(auth_id))
            .get_result::<LockerAuthInfo>(&mut conn)?;

        Ok(result)
    }

    fn delete(
            &self,
            auth_id: Uuid,
        ) -> Result<usize, RepositoryError> {
        let mut conn = self.pool.get()?;
        let result = diesel::delete(locker_auth_info::table.find(auth_id))
            .execute(&mut conn)?;

        Ok(result)
    }

    fn delete_all(
            &self
        ) -> Result<usize, RepositoryError> {
        let mut conn = self.pool.get()?;
        let result = diesel::delete(locker_auth_info::table)
            .execute(&mut conn)?;

        Ok(result)
    }
}
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;
use async_trait::async_trait;

use crate::infrastructure::schema::*;
use crate::infrastructure::models::*;
use crate::infrastructure::router::Pool;

/// # locker_auth_info
#[async_trait]
pub trait LockerAuthInfoRepository: Send + Sync {
    async fn insert(
        &self,
        auth_id: &Uuid,
        main_student_id: &String,
        main_family_name: &String,
        main_given_name: &String,
        co_student_id: &String,
        co_family_name: &String,
        co_given_name: &String,
    ) -> Result<LockerAuthInfo, Error>;
    async fn get_by_id(
        &self,
        auth_id: &Uuid,
    ) -> Result<LockerAuthInfo, Error>;
    async fn delete(
        &self,
        auth_id: &Uuid,
    ) -> Result<usize, Error>;
    async fn delete_all(
        &self
    ) -> Result<usize, Error>;
}

pub struct LockerAuthInfoRepositorySqlImpl {
    pool: Pool<PgConnection>
}

impl LockerAuthInfoRepositorySqlImpl {
    pub fn new(pool: Pool<PgConnection>) -> Self {
        LockerAuthInfoRepositorySqlImpl { pool }
    }
}

#[async_trait]
impl LockerAuthInfoRepository for LockerAuthInfoRepositorySqlImpl {
    async fn insert(
            &self,
            auth_id: &Uuid,
            main_student_id: &String,
            main_family_name: &String,
            main_given_name: &String,
            co_student_id: &String,
            co_family_name: &String,
            co_given_name: &String,
    ) -> Result<LockerAuthInfo, Error> {
        let new_auth_info = NewLockerAuthInfo {
            auth_id: auth_id,
            main_student_id: main_student_id,
            main_family_name: main_family_name,
            main_given_name: main_given_name,
            co_student_id: co_student_id,
            co_family_name: co_family_name,
            co_given_name: co_given_name,
        };
        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(locker_auth_info::table)
            .values(&new_auth_info)
            .get_result(&mut conn)
    }
    async fn get_by_id(
            &self,
            auth_id: &Uuid,
        ) -> Result<LockerAuthInfo, Error> {
            let mut conn = self.pool.get().unwrap();
        locker_auth_info::table.filter(locker_auth_info::auth_id.eq(auth_id))
            .get_result(&mut conn)
    }
    async fn delete(
            &self,
            auth_id: &Uuid,
        ) -> Result<usize, Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::delete(locker_auth_info::table.find(auth_id))
            .execute(&mut conn)
    }
    async fn delete_all(
            &self
        ) -> Result<usize, Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::delete(locker_auth_info::table)
            .execute(&mut conn)
    }
}
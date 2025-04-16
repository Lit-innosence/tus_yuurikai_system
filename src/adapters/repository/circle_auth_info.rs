use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid::Uuid;

use crate::infrastructure::schema::*;
use crate::infrastructure::models::*;
use crate::infrastructure::router::Pool;
use super::RepositoryError;

/// # circle_auth_info
pub trait CircleAuthInfoRepository: Send + Sync {
    fn insert(
        &self,
        auth_id: Uuid,
        main_student_id: String,
        main_family_name: String,
        main_given_name: String,
        main_email: String,
        main_phone: String,
        co_student_id: String,
        co_family_name: String,
        co_given_name: String,
        co_email: String,
        co_phone: String,
        b_url: String,
        c_url: String,
        d_url: String,
        organization_name: String,
        organization_ruby: String,
        organization_email: String,
    ) -> Result<CircleAuthInfo, RepositoryError>;

    fn get_by_id(
        &self,
        auth_id: Uuid,
    ) -> Result<CircleAuthInfo, RepositoryError>;

    fn delete(
        &self,
        auth_id: Uuid,
    ) -> Result<usize, RepositoryError>;

    fn delete_all(
        &self
    ) -> Result<usize, RepositoryError>;
}

pub struct CircleAuthInfoRepositorySqlImpl {
    pool: Pool<PgConnection>
}

impl CircleAuthInfoRepositorySqlImpl {
    pub fn new(pool: Pool<PgConnection>) -> Self {
        CircleAuthInfoRepositorySqlImpl { pool }
    }
}

impl CircleAuthInfoRepository for CircleAuthInfoRepositorySqlImpl {
    fn insert(
            &self,
            auth_id: Uuid,
            main_student_id: String,
            main_family_name: String,
            main_given_name: String,
            main_email: String,
            main_phone: String,
            co_student_id: String,
            co_family_name: String,
            co_given_name: String,
            co_email: String,
            co_phone: String,
            b_doc: String,
            c_doc: String,
            d_doc: String,
            organization_name: String,
            organization_ruby: String,
            organization_email: String,
    ) -> Result<CircleAuthInfo, RepositoryError> {
        let new_auth_info = NewCircleAuthInfo {
            auth_id: &auth_id,
            main_student_id: &main_student_id,
            main_family_name: &main_family_name,
            main_given_name: &main_given_name,
            main_email: &main_email,
            main_phone: &main_phone,
            co_student_id: &co_student_id,
            co_family_name: &co_family_name,
            co_given_name: &co_given_name,
            co_email: &co_email,
            co_phone: &co_phone,
            b_doc: &b_doc,
            c_doc: &c_doc,
            d_doc: &d_doc,
            organization_name: &organization_name,
            organization_ruby: &organization_ruby,
            organization_email: &organization_email,
        };
        let mut conn = self.pool.get()?;
        let result = diesel::insert_into(circle_auth_info::table)
            .values(&new_auth_info)
            .get_result::<CircleAuthInfo>(&mut conn)?;

        Ok(result)
    }

    fn get_by_id(
            &self,
            auth_id: Uuid,
        ) -> Result<CircleAuthInfo, RepositoryError> {
            let mut conn = self.pool.get()?;
        let result = circle_auth_info::table.filter(circle_auth_info::auth_id.eq(auth_id))
            .get_result::<CircleAuthInfo>(&mut conn)?;

        Ok(result)
    }

    fn delete(
            &self,
            auth_id: Uuid,
        ) -> Result<usize, RepositoryError> {
        let mut conn = self.pool.get()?;
        let result = diesel::delete(circle_auth_info::table.find(auth_id))
            .execute(&mut conn)?;

        Ok(result)
    }

    fn delete_all(
            &self
        ) -> Result<usize, RepositoryError> {
        let mut conn = self.pool.get()?;
        let result = diesel::delete(circle_auth_info::table)
            .execute(&mut conn)?;

        Ok(result)
    }
}
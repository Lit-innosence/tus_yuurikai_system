use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use async_trait::async_trait;

use crate::infrastructure::schema::*;
use crate::infrastructure::models::*;
use crate::infrastructure::router::Pool;

/// # registration
#[async_trait]
pub trait RegistrationRepository: Send + Sync {
    async fn insert(
        &self,
        organization_id: &i32,
        year: &i32,
        main_student_id: &String,
        co_student_id: &String,
        status_acceptance: &String,
        status_authentication: &String,
        status_form_confirmation: &String,
        status_registration_complete: &String,
        b_url: &String,
        c_url: &String,
        d_url: &String,
    ) -> Result<Registration, Error>;

    async fn update_student_by_id (
        &self,
        organization_id: &i32,
        main_student_id: &String,
        co_student_id: &String,
    ) -> Result<Registration, Error>;

}

pub struct RegistrationRepositorySqlImpl {
    pool: Pool<PgConnection>
}

impl RegistrationRepositorySqlImpl {
    pub fn new(pool: Pool<PgConnection>) -> Self {
        RegistrationRepositorySqlImpl { pool }
    }
}

#[async_trait]
impl RegistrationRepository for RegistrationRepositorySqlImpl {
    async fn insert(
            &self,
            organization_id: &i32,
            year: &i32,
            main_student_id: &String,
            co_student_id: &String,
            status_acceptance: &String,
            status_authentication: &String,
            status_form_confirmation: &String,
            status_registration_complete: &String,
            b_doc: &String,
            c_doc: &String,
            d_doc: &String,
        ) -> Result<Registration, Error> {
        let mut conn = self.pool.get().unwrap();
        let new_registration = NewRegistration{
            organization_id: organization_id,
            year: year,
            main_student_id: main_student_id,
            co_student_id: co_student_id,
            status_acceptance: status_acceptance,
            status_authentication:status_authentication,
            status_form_confirmation: status_form_confirmation,
            status_registration_complete: status_registration_complete,
            b_doc: b_doc,
            c_doc: c_doc,
            d_doc: d_doc,
        };
        diesel::insert_into(registration::table)
            .values(new_registration)
            .get_result(&mut conn)
    }

    async fn update_student_by_id (
            &self,
            organization_id: &i32,
            main_student_id: &String,
            co_student_id: &String,
        ) -> Result<Registration, Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::update(registration::table)
            .filter(registration::organization_id.eq(organization_id))
            .set((registration::main_student_id.eq(main_student_id), registration::co_student_id.eq(co_student_id), registration::updated_at.eq(diesel::dsl::now)))
            .get_result(&mut conn)
    }
}
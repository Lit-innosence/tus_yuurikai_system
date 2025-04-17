use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::infrastructure::schema::*;
use crate::infrastructure::models::*;
use crate::infrastructure::router::Pool;
use super::RepositoryError;

/// # registration
pub trait RegistrationRepository: Send + Sync {
    fn insert(
        &self,
        organization_id: i32,
        year: i32,
        main_student_id: String,
        co_student_id: String,
        status_acceptance: String,
        status_authentication: String,
        status_form_confirmation: String,
        status_registration_complete: String,
        b_url: String,
        c_url: String,
        d_url: String,
    ) -> Result<Registration, RepositoryError>;

    fn update_student_by_id (
        &self,
        organization_id: i32,
        main_student_id: String,
        co_student_id: String,
    ) -> Result<Registration, RepositoryError>;

    fn update_status_by_id (
        &self,
        organization_id: i32,
        status_acceptance: String,
        status_authentication: String,
        status_form_confirmation: String,
        status_registration_complete: String,
    ) -> Result<Registration, RepositoryError>;

    fn get_all (
        &self,
    ) -> Result<Vec<Registration>, RepositoryError>;


}

pub struct RegistrationRepositorySqlImpl {
    pool: Pool<PgConnection>
}

impl RegistrationRepositorySqlImpl {
    pub fn new(pool: Pool<PgConnection>) -> Self {
        RegistrationRepositorySqlImpl { pool }
    }
}

impl RegistrationRepository for RegistrationRepositorySqlImpl {
    fn insert(
            &self,
            organization_id: i32,
            year: i32,
            main_student_id: String,
            co_student_id: String,
            status_acceptance: String,
            status_authentication: String,
            status_form_confirmation: String,
            status_registration_complete: String,
            b_doc: String,
            c_doc: String,
            d_doc: String,
        ) -> Result<Registration, RepositoryError> {
        let mut conn = self.pool.get()?;
        let new_registration = NewRegistration{
            organization_id: &organization_id,
            year: &year,
            main_student_id: &main_student_id,
            co_student_id: &co_student_id,
            status_acceptance: &status_acceptance,
            status_authentication: &status_authentication,
            status_form_confirmation: &status_form_confirmation,
            status_registration_complete: &status_registration_complete,
            b_doc: &b_doc,
            c_doc: &c_doc,
            d_doc: &d_doc,
        };
        let result = diesel::insert_into(registration::table)
            .values(new_registration)
            .get_result::<Registration>(&mut conn)?;

        Ok(result)
    }

    fn update_student_by_id (
            &self,
            organization_id: i32,
            main_student_id: String,
            co_student_id: String,
        ) -> Result<Registration, RepositoryError> {
        let mut conn = self.pool.get()?;
        let result = diesel::update(registration::table)
            .filter(registration::organization_id.eq(organization_id))
            .set((registration::main_student_id.eq(main_student_id), registration::co_student_id.eq(co_student_id), registration::updated_at.eq(diesel::dsl::now)))
            .get_result::<Registration>(&mut conn)?;

        Ok(result)
    }

    fn update_status_by_id (
            &self,
            organization_id: i32,
            status_acceptance: String,
            status_authentication: String,
            status_form_confirmation: String,
            status_registration_complete: String,
        ) -> Result<Registration, RepositoryError> {
        let mut conn = self.pool.get()?;
        let result = diesel::update(registration::table)
            .filter(registration::organization_id.eq(organization_id))
            .set((registration::status_acceptance.eq(status_acceptance), registration::status_authentication.eq(status_authentication), registration::status_form_confirmation.eq(status_form_confirmation), registration::status_registration_complete.eq(status_registration_complete)))
            .get_result::<Registration>(&mut conn)?;

        Ok(result)
    }

    fn get_all (
            &self,
        ) -> Result<Vec<Registration>, RepositoryError> {
        let mut conn = self.pool.get()?;
        let result = registration::table.get_results::<Registration>(&mut conn)?;

        Ok(result)
    }
}
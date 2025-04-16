use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::infrastructure::schema::*;
use crate::infrastructure::models::*;
use crate::infrastructure::router::Pool;
use super::RepositoryError;

/// # representatives
pub trait RepresentativesRepository: Send + Sync {
    fn insert(
        &self,
        student_id: String,
        family_name: String,
        given_name: String,
        email: String,
        phone: String,
    ) -> Result<Representatives, RepositoryError>;

    fn get_all(
        &self,
    ) -> Result<Vec<Representatives>, RepositoryError>;

    fn get_by_id(
        &self,
        student_id: String,
    ) -> Result<Representatives, RepositoryError>;
}

pub struct RepresentativesRepositorySqlImpl {
    pool: Pool<PgConnection>
}

impl RepresentativesRepositorySqlImpl {
    pub fn new(pool: Pool<PgConnection>) -> Self {
        RepresentativesRepositorySqlImpl { pool }
    }
}

impl RepresentativesRepository for RepresentativesRepositorySqlImpl {
    fn insert(
            &self,
            student_id: String,
            family_name: String,
            given_name: String,
            email: String,
            phone: String,
        ) -> Result<Representatives, RepositoryError> {
        let new_representative = NewRepresentatives{
            student_id: &student_id,
            family_name: &family_name,
            given_name: &given_name,
            email: &email.clone(),
            phone: &phone.clone(),
        };
        let mut conn = self.pool.get()?;
        let result = diesel::insert_into(representatives::table)
            .values(new_representative)
            .on_conflict(representatives::student_id)
            .do_update()
            .set((representatives::updated_at.eq(diesel::dsl::now), representatives::email.eq(email), representatives::phone.eq(phone)))
            .get_result::<Representatives>(&mut conn)?;

        Ok(result)
    }

    fn get_all(
            &self,
        ) -> Result<Vec<Representatives>, RepositoryError> {
        let mut conn = self.pool.get()?;
        let result = representatives::table
            .get_results::<Representatives>(&mut conn)?;

        Ok(result)
    }

    fn get_by_id(
            &self,
            student_id: String,
        ) -> Result<Representatives, RepositoryError> {
        let mut conn = self.pool.get()?;
        let result = representatives::table
            .filter(representatives::student_id.eq(student_id))
            .get_result::<Representatives>(&mut conn)?;

        Ok(result)
    }
}
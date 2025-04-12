use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

use crate::infrastructure::schema::*;
use crate::infrastructure::models::*;
use crate::infrastructure::router::Pool;

/// # representatives
pub trait RepresentativesRepository: Send + Sync {
    fn insert(
        &self,
        student_id: String,
        family_name: String,
        given_name: String,
        email: String,
        phone: String,
    ) -> Result<Representatives, Error>;

    fn get_all(
        &self,
    ) -> Result<Vec<Representatives>, Error>;

    fn get_by_id(
        &self,
        student_id: String,
    ) -> Result<Representatives, Error>;
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
        ) -> Result<Representatives, Error> {
        let new_representative = NewRepresentatives{
            student_id: &student_id,
            family_name: &family_name,
            given_name: &given_name,
            email: &email.clone(),
            phone: &phone.clone(),
        };
        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(representatives::table)
            .values(new_representative)
            .on_conflict(representatives::student_id)
            .do_update()
            .set((representatives::updated_at.eq(diesel::dsl::now), representatives::email.eq(email), representatives::phone.eq(phone)))
            .get_result(&mut conn)
    }

    fn get_all(
            &self,
        ) -> Result<Vec<Representatives>, Error> {
        let mut conn = self.pool.get().unwrap();
        representatives::table
            .get_results(&mut conn)
    }

    fn get_by_id(
            &self,
            student_id: String,
        ) -> Result<Representatives, Error> {
        let mut conn = self.pool.get().unwrap();
        representatives::table
            .filter(representatives::student_id.eq(student_id))
            .get_result(&mut conn)
    }
}
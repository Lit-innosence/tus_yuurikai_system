use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

use crate::infrastructure::schema::*;
use crate::infrastructure::models::*;
use crate::infrastructure::router::Pool;


/// # student
pub trait StudentRepository: Send + Sync {
    fn insert(
        &self,
        student_id: String,
        family_name: String,
        given_name: String,
    ) -> Result<Student, Error>;

    fn get_all(
        &self
    ) -> Result<Vec<Student>, Error>;

     fn get_by_id(
        &self,
        student_id: String,
    ) -> Result<Student, Error>;

     fn get_by_name(
        &self,
        family_name: String,
        given_name: String,
    ) -> Result<Vec<Student>, Error>;

     fn delete_all(
        &self
    ) -> Result<usize, Error>;
}
pub struct StudentRepositorySqlImpl {
    pool: Pool<PgConnection>
}

impl StudentRepositorySqlImpl {
    pub fn new(pool: Pool<PgConnection>) -> Self {
        StudentRepositorySqlImpl { pool }
    }
}

impl StudentRepository for StudentRepositorySqlImpl{
     fn insert(
        &self,
        student_id: String,
        family_name: String,
        given_name: String,
    ) -> Result<Student, Error> {
        let new_student = NewStudent {
            student_id: &student_id,
            family_name: &family_name,
            given_name: &given_name,
        };
        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(student::table)
            .values(&new_student)
            .on_conflict(student::student_id)
            .do_update()
            .set(student::updated_at.eq(diesel::dsl::now))
            .get_result(&mut conn)
    }

     fn get_all(
        &self
    ) -> Result<Vec<Student>, Error> {
        let mut conn = self.pool.get().unwrap();
        student::table
            .get_results(&mut conn)
    }

     fn get_by_id(
            &self,
            student_id: String,
        ) -> Result<Student, Error> {
        let mut conn = self.pool.get().unwrap();
        student::table
            .filter(student::student_id.eq(student_id))
            .get_result(&mut conn)
    }

     fn get_by_name(
        &self,
        family_name: String,
        given_name: String,
    ) -> Result<Vec<Student>, Error> {
        let mut conn = self.pool.get().unwrap();
        let family_name_ex = format!("{}%", family_name);
        let given_name_ex = format!("{}%", given_name);

        student::table
            .filter(student::family_name.like(family_name_ex).and(student::given_name.like(given_name_ex)))
            .get_results(&mut conn)
    }

     fn delete_all(
        &self
    ) -> Result<usize, Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::delete(student::table)
            .execute(&mut conn)
    }
}
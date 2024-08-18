use super::models::*;
use super::schema::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use dotenv::dotenv;
use std::env;

pub fn create_connection() -> Result<PgConnection, ConnectionError> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    PgConnection::establish(&database_url)
    
}

pub fn insert_student(
    conn:     &mut PgConnection, 
    student_id: &String,
    family_name: &String,
    given_name: &String) -> Result<Student, Error> {

    let new_student = NewStudent { student_id, family_name, given_name };
    diesel::insert_into(student::table)
            .values(&new_student)
            .on_conflict(student::student_id)
            .do_update()
            .set(student::updated_at.eq(diesel::dsl::now))
            .get_result(conn)

}

pub fn insert_studentpair(
    conn:     &mut PgConnection, 
    student_id1 : &String,
    student_id2 : &String,
    year: &i32) -> Result<StudentPair, Error> {

    let new_studentpair = NewStudentPair { student_id1, student_id2, year };
    diesel::insert_into(student_pair::table)
            .values(&new_studentpair)
            .get_result(conn)

}
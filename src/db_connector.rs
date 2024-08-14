use super::models::*;
use super::schema::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use dotenv::dotenv;
use std::env;

pub fn create_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    
}

pub fn insert_student(
    conn:     &mut PgConnection, 
    student_id: &String,
    family_name: &String,
    given_name: &String) -> Result<Student, Error> {

let new_student = NewStudent { student_id, family_name, given_name };
diesel::insert_into(student::table)
    .values(&new_student)
    .get_result(conn)

}
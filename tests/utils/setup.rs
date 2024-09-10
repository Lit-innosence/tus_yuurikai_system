use std::env;
use dotenv::dotenv;
use diesel::prelude::*;

use tus_yuurikai_system::infrastracture::schema::{assignment_record, student, student_pair};

pub fn setup_db() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let mut conn = PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connectiong to {}", database_url));

    let _ = diesel::delete(student::table)
        .execute(&mut conn);
    let _ = diesel::delete(student_pair::table)
        .execute(&mut conn);
    let _ = diesel::delete(assignment_record::table)
        .execute(&mut conn);
}
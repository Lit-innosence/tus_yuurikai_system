use diesel::{Insertable, Queryable};
use crate::infrastracture::schema::{student, student_pair, locker, assignment_record, auth};
// student

#[derive(Queryable)]
pub struct Student {
    pub student_id: String,
    pub family_name: String,
    pub given_name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "student"]
pub struct NewStudent<'a> {
    pub student_id: &'a String,
    pub family_name: &'a String,
    pub given_name: &'a String,
}

// student_pair

#[derive(Queryable)]
pub struct StudentPair {
    pub pair_id: uuid::Uuid,
    pub student_id1: String,
    pub student_id2: String,
    pub year: i32,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "student_pair"]
pub struct NewStudentPair<'a> {
    pub student_id1: &'a String,
    pub student_id2: &'a String,
    pub year: &'a i32,
}

// locker

#[derive(Queryable)]
pub struct Locker {
    pub locker_id: String,
    pub location: String,
}

#[derive(Insertable)]
#[table_name = "locker"]
pub struct NewLocker<'a> {
    pub locker_id: &'a String,
    pub location: &'a String,
}

// assignment_record

#[derive(Queryable)]
pub struct AssignmentRecord {
    pub record_id: uuid::Uuid,
    pub pair_id: uuid::Uuid,
    pub locker_id: String,
    pub year: i32,
    pub created_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "assignment_record"]
pub struct NewAssignmentRecord<'a> {
    pub pair_id: &'a uuid::Uuid,
    pub locker_id: &'a String,
    pub year: &'a i32,
}

// auth

#[derive(Queryable)]
pub struct Auth {
    pub auth_token: String,
    pub student_id: String,
    pub family_name: String,
    pub given_name: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "auth"]
pub struct NewAuth<'a> {
    pub auth_token: &'a String,
    pub student_id: &'a String,
    pub family_name: &'a String,
    pub given_name: &'a String,
}

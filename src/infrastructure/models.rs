use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use crate::infrastructure::schema::{student, student_pair, locker, assignment_record, auth, admin};
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
#[diesel(table_name = student)]
pub struct NewStudent<'a> {
    pub student_id: &'a String,
    pub family_name: &'a String,
    pub given_name: &'a String,
}

// student_pair

#[derive(PartialEq, Eq, Hash, Queryable)]
pub struct StudentPair {
    pub pair_id: uuid::Uuid,
    pub student_id1: String,
    pub student_id2: String,
    pub year: i32,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = student_pair)]
pub struct NewStudentPair<'a> {
    pub student_id1: &'a String,
    pub student_id2: &'a String,
    pub year: &'a i32,
}

// locker

#[derive(Debug, PartialEq, Eq, Queryable, Serialize, Deserialize)]
pub struct Locker {
    pub locker_id: String,
    pub location: String,
    pub status: String,
}

#[derive(Insertable)]
#[diesel(table_name = locker)]
pub struct NewLocker<'a> {
    pub locker_id: &'a String,
    pub location: &'a String,
    pub status: &'a String,
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
#[diesel(table_name = assignment_record)]
pub struct NewAssignmentRecord<'a> {
    pub pair_id: &'a uuid::Uuid,
    pub locker_id: &'a String,
    pub year: &'a i32,
}

// auth

#[derive(Queryable)]
pub struct Auth {
    pub main_auth_token: String,
    pub main_student_id: String,
    pub main_family_name: String,
    pub main_given_name: String,
    pub co_auth_token: String,
    pub co_student_id: String,
    pub co_family_name: String,
    pub co_given_name: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = auth)]
pub struct NewAuth<'a> {
    pub main_auth_token: &'a String,
    pub main_student_id: &'a String,
    pub main_family_name: &'a String,
    pub main_given_name: &'a String,
    pub co_auth_token: &'a String,
    pub co_student_id: &'a String,
    pub co_family_name: &'a String,
    pub co_given_name: &'a String,
}

#[derive(Queryable)]
pub struct AssignmentRecordGetResult {
    pub pair_id: uuid::Uuid,
    pub student_id1: String,
    pub student_id2: String,
    pub record_id: uuid::Uuid,
    pub locker_id: String,
    pub year: i32,
}

#[derive(Queryable)]
pub struct Admin{
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = admin)]
pub struct NewAdmin<'a> {
    pub username: &'a String,
    pub password: &'a String,
}
use crate::schema::*;
use diesel::{Insertable, Queryable};
use uuid::Uuid;

#[derive(Queryable)]
pub struct Student {
    pub student_id: String,
    pub family_name: String,
    pub given_name: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Queryable)]
pub struct StudentPair {
    pub pair_id:  	Uuid,
    pub student_id1: String,
    pub student_id2: String,
    pub year: i32,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable)]
pub struct Locker {
    pub locker_id: String,
    pub location: String,
}

#[derive(Queryable)]
pub struct AssginmentRecord {
    pub record_id: Uuid,
    pub pair_id: Strubg,
    pub locker_id: String,
    pub year: i32,
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

#[derive(Insertable)]
#[table_name = "student_pair"]
pub struct NewStudentPair<'a>{
    pub student_id1: &'a String,
    pub student_id2: &'a String,
    pub year: &'a i32
}

#[derive(Insertable)]
#[table_name = "locker"]
pub struct NewLocker<'a>{
    pub locker_id: &'a String,
    pub location: &'a String,
}

#[derive(Insertable)]
#[table_name = "assignment_record"]
pub struct NewAssignmentRecord<'a>{
    pub pair_id: &'a Uuid,
    pub locker_id: &'a String,
    pub year: &'a i32,
}
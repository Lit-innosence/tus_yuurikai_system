use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;
use async_trait::async_trait;

use crate::infrastracture::schema::{student, student_pair, locker, assignment_record, auth};
use crate::infrastracture::models::{Student, NewStudent, StudentPair, NewStudentPair, Auth, NewAuth, Locker, NewLocker ,AssignmentRecord, NewAssignmentRecord};
use crate::infrastracture::router::Pool;

// student
#[async_trait]
pub trait StudentRepository: Send + Sync {
    async fn insert (
        &self,
        student_id: &String,
        family_name: &String,
        given_name: &String,
    ) -> Result<Student, Error>;
}
pub struct StudentRepositorySqlImpl {
    pool: Pool<PgConnection>
}

impl StudentRepositorySqlImpl {
    pub fn new(pool: Pool<PgConnection>) -> Self {
        StudentRepositorySqlImpl { pool }
    }
}

#[async_trait]
impl StudentRepository for StudentRepositorySqlImpl{
    async fn insert(
        &self,
        student_id: &String,
        family_name: &String,
        given_name: &String,
    ) -> Result<Student, Error> {
        let new_student = NewStudent {
            student_id,
            family_name,
            given_name,
        };
        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(student::table)
            .values(&new_student)
            .on_conflict(student::student_id)
            .do_update()
            .set(student::updated_at.eq(diesel::dsl::now))
            .get_result(&mut conn)
    }
}

// student_pair

#[async_trait]
pub trait StudentPairRepository: Send + Sync {
    async fn insert(
        &self,
        student_id1: &String,
        student_id2: &String,
        year: &i32,
    ) -> Result<StudentPair, Error>;
    async fn get_by_student_id_and_year(
        &self,
        student_id: &String,
        year: &i32,
    ) -> Result<StudentPair, Error>;
}

pub struct StudentPairRepositorySqlImpl{
    pool: Pool<PgConnection>
}

impl StudentPairRepositorySqlImpl {
    pub fn new(pool: Pool<PgConnection>) -> Self {
        StudentPairRepositorySqlImpl { pool }
    }
}

#[async_trait]
impl StudentPairRepository for StudentPairRepositorySqlImpl{
    async fn insert(
        &self,
        student_id1: &String,
        student_id2: &String,
        year: &i32,
    ) -> Result<StudentPair, Error> {
        let new_studentpair = NewStudentPair {
            student_id1,
            student_id2,
            year,
        };
        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(student_pair::table)
            .values(&new_studentpair)
            .get_result(&mut conn)
    }

    async fn get_by_student_id_and_year(
        &self,
        student_id: &String,
        year: &i32,
    ) -> Result<StudentPair, Error> {
        let mut conn = self.pool.get().unwrap();
        student_pair::table
            .filter(
                student_pair::student_id1
                    .eq(student_id)
                    .and(student_pair::year.eq(year)),
            )
            .first(&mut conn)
    }
}


// auth

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn insert (
    &self,
    auth_token: &String,
    student_id: &String,
    family_name: &String,
    given_name: &String,
    ) -> Result<Auth, Error>;
}

pub struct AuthRepositorySqlImpl {
    pool: Pool<PgConnection>
}

impl AuthRepositorySqlImpl {
    pub fn new(pool: Pool<PgConnection>) -> Self {
        AuthRepositorySqlImpl { pool }
    }
}

#[async_trait]
impl AuthRepository for AuthRepositorySqlImpl {
    async fn insert(
        &self,
        auth_token: &String,
        student_id: &String,
        family_name: &String,
        given_name: &String,
    ) -> Result<Auth, Error> {
        let new_auth = NewAuth {
            auth_token,
            student_id,
            family_name,
            given_name,
        };
        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(auth::table)
            .values(&new_auth)
            .get_result(&mut conn)
    }
}

// locker

#[async_trait]
pub trait LockerRepository: Send + Sync {
    async fn insert(
        &self,
        locker_id: &String,
        location: &String,
    ) -> Result<Locker, Error>;
}

pub struct LockerRepositorySqlImpl {
    pool: Pool<PgConnection>
}

impl LockerRepositorySqlImpl {
    pub fn new(pool: Pool<PgConnection>) -> Self {
        LockerRepositorySqlImpl { pool }
    }
}

#[async_trait]
impl LockerRepository for LockerRepositorySqlImpl {
    async fn insert(
        &self,
        locker_id: &String,
        location: &String,
    ) -> Result<Locker, Error> {
        let new_locker = NewLocker {
            locker_id,
            location,
        };
        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(locker::table)
            .values(&new_locker)
            .get_result(&mut conn)
    }
}

// assignment_record

#[async_trait]
pub trait AssignmentRecordRepository: Send + Sync {
    async fn insert(
        &self,
        pair_id: &Uuid,
        locker_id: &String,
        year: &i32,
    ) -> Result<AssignmentRecord, Error>;
}

pub struct AssignmentRecordRepositorySqlImpl {
    pool : Pool<PgConnection>
}

impl AssignmentRecordRepositorySqlImpl {
    pub fn new(pool: Pool<PgConnection>) -> Self {
        AssignmentRecordRepositorySqlImpl { pool }
    }
}

#[async_trait]
impl AssignmentRecordRepository for AssignmentRecordRepositorySqlImpl {
    async fn insert (
        &self,
        pair_id: &Uuid,
        locker_id: &String,
        year: &i32,
    ) -> Result<AssignmentRecord, Error> {
        let new_assignmentrecord = NewAssignmentRecord {
            pair_id,
            locker_id,
            year,
        };
        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(assignment_record::table)
            .values(&new_assignmentrecord)
            .get_result(&mut conn)
    }
}
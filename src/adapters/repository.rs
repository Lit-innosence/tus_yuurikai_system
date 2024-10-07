use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;
use async_trait::async_trait;

use crate::infrastracture::schema::{student, student_pair, locker, assignment_record, auth};
use crate::infrastracture::models::{Student, NewStudent, StudentPair, NewStudentPair, Auth, NewAuth, Locker, NewLocker ,AssignmentRecord, NewAssignmentRecord, AssignmentRecordGetResult};
use crate::infrastracture::router::Pool;

// student

#[async_trait]
pub trait StudentRepository: Send + Sync {
    async fn insert(
        &self,
        student_id: &String,
        family_name: &String,
        given_name: &String,
    ) -> Result<Student, Error>;

    async fn get_by_id(
        &self,
        student_id: &String,
    ) -> Result<Student, Error>;

    async fn get_by_name(
        &self,
        family_name: &String,
        given_name: &String,
    ) -> Result<Student, Error>;

    async fn delete_all(
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

    async fn get_by_id(
            &self,
            student_id: &String,
        ) -> Result<Student, Error> {
        let mut conn = self.pool.get().unwrap();
        student::table
            .filter(student::student_id.eq(student_id))
            .get_result(&mut conn)
    }

    async fn get_by_name(
        &self,
        family_name: &String,
        given_name: &String,
    ) -> Result<Student, Error> {
        let mut conn = self.pool.get().unwrap();
        student::table
            .filter(student::family_name.eq(family_name).and(student::given_name.eq(given_name)))
            .first(&mut conn)
    }

    async fn delete_all(
        &self
    ) -> Result<usize, Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::delete(student::table)
            .execute(&mut conn)
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
    async fn delete_all(
        &self
    ) -> Result<usize, Error>;
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

    async fn delete_all(
        &self
    ) -> Result<usize, Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::delete(student_pair::table)
            .execute(&mut conn)
    }
}


// auth

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn insert (
    &self,
    main_auth_token: &String,
    main_student_id: &String,
    main_family_name: &String,
    main_given_name: &String,
    co_auth_token: &String,
    co_student_id: &String,
    co_family_name: &String,
    co_given_name: &String,
    ) -> Result<Auth, Error>;
    async fn get_by_token(
        &self,
        auth_token: &String,
    ) -> Result<Auth, Error>;
    async fn delete_all(
        &self
    ) -> Result<usize, Error>;
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
        main_auth_token: &String,
        main_student_id: &String,
        main_family_name: &String,
        main_given_name: &String,
        co_auth_token: &String,
        co_student_id: &String,
        co_family_name: &String,
        co_given_name: &String,
    ) -> Result<Auth, Error> {
        let new_auth = NewAuth {
            main_auth_token,
            main_student_id,
            main_family_name,
            main_given_name,
            co_auth_token,
            co_student_id,
            co_family_name,
            co_given_name,
        };
        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(auth::table)
            .values(&new_auth)
            .get_result(&mut conn)
    }
    async fn get_by_token(
        &self,
        auth_token: &String,
    ) -> Result<Auth, Error> {
        let mut conn = self.pool.get().unwrap();
        auth::table
            .filter(
                auth::main_auth_token
                    .eq(auth_token)
                    .or(auth::co_auth_token.eq(auth_token)),
            )
            .first(&mut conn)
    }

    async fn delete_all(
        &self
    ) -> Result<usize, Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::delete(auth::table)
            .execute(&mut conn)
    }
}

// locker

#[async_trait]
pub trait LockerRepository: Send + Sync {
    async fn insert(
        &self,
        locker_id: &String,
        location: &String,
        status: &String,
    ) -> Result<Locker, Error>;

    async fn update_status(
        &self,
        locker_id: &String,
        status: &String,
    ) -> Result<usize, Error>;

    async fn update_all_status(
        &self,
        status: &String,
    ) -> Result<usize, Error>;

    async fn get_by_id(
        &self,
        locker_id: &String,
    ) -> Result<Locker, Error>;

    async fn delete_all(
        &self
    ) -> Result<usize, Error>;
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
        status: &String,
    ) -> Result<Locker, Error> {
        let new_locker = NewLocker {
            locker_id,
            location,
            status
        };
        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(locker::table)
            .values(&new_locker)
            .get_result(&mut conn)
    }

    async fn update_status(
            &self,
            locker_id: &String,
            status: &String,
        ) -> Result<usize, Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::update(locker::table.find(locker_id))
            .set(locker::status.eq(status))
            .execute(&mut conn)
    }

    async fn update_all_status(
            &self,
            status: &String,
        ) -> Result<usize, Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::update(locker::table)
            .set(locker::status.eq(status))
            .execute(&mut conn)
    }

    async  fn get_by_id(
            &self,
            locker_id: &String,
        ) -> Result<Locker, Error> {
            let mut conn = self.pool.get().unwrap();
        locker::table
            .filter(
                locker::locker_id
                .eq(locker_id)
            ).first(&mut conn)
    }

    async fn delete_all(
        &self
    ) -> Result<usize, Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::delete(locker::table)
            .execute(&mut conn)
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

    async fn get(
        &self,
        year: &i32,
        floor: &String,
        pair_id: &Uuid,
    ) -> Result<Vec<AssignmentRecordGetResult>, Error>;

    async fn delete_all(
        &self
    ) -> Result<usize, Error>;
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

    async fn get (
        &self,
        year: &i32,
        floor: &String,
        pair_id: &Uuid,
    ) -> Result<Vec<AssignmentRecordGetResult>, Error> {
        let mut conn = self.pool.get().unwrap();

        let floor_ex = format!("{}%", floor);

        student_pair::table
            .select((student_pair::pair_id, student_pair::student_id1, student_pair::student_id2, student_pair::year)
            ).inner_join(assignment_record::table)
            .select((student_pair::pair_id, student_pair::student_id1, student_pair::student_id2, assignment_record::record_id, assignment_record::locker_id, assignment_record::year))
            .filter(assignment_record::locker_id
                .like(floor_ex)
            ).filter(student_pair::pair_id.eq(pair_id).and(assignment_record::year.eq(year))
            ).get_results(&mut conn)
    }

    async fn delete_all(
        &self
    ) -> Result<usize, Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::delete(assignment_record::table)
            .execute(&mut conn)
    }
}
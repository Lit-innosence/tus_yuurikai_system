use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;
use async_trait::async_trait;

use crate::infrastructure::schema::*;
use crate::infrastructure::models::*;
use crate::infrastructure::router::Pool;

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
    ) -> Result<Vec<Student>, Error>;

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
    ) -> Result<Vec<Student>, Error> {
        let mut conn = self.pool.get().unwrap();
        let family_name_ex = format!("{}%", family_name);
        let given_name_ex = format!("{}%", given_name);

        student::table
            .filter(student::family_name.like(family_name_ex).and(student::given_name.like(given_name_ex)))
            .get_results(&mut conn)
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
    async fn get_by_main_id_and_year(
        &self,
        student_id: &String,
        year: &i32,
    ) -> Result<StudentPair, Error>;
    async fn get_by_pair_id_and_year(
        &self,
        pair_id : &Uuid,
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
                    .or(student_pair::student_id2.eq(student_id))
                    .and(student_pair::year.eq(year))
            )
            .first(&mut conn)
    }

    async fn get_by_main_id_and_year(
        &self,
        student_id: &String,
        year: &i32,
    ) -> Result<StudentPair, Error> {
        let mut conn = self.pool.get().unwrap();
        student_pair::table
            .filter(
                student_pair::student_id1
                    .eq(student_id)
                    .and(student_pair::year.eq(year))
            )
            .first(&mut conn)
    }

    async fn get_by_pair_id_and_year(
        &self,
        pair_id: &Uuid,
        year: &i32,
    ) -> Result<StudentPair, Error> {
        let mut conn = self.pool.get().unwrap();
        student_pair::table
            .filter(
                student_pair::pair_id
                    .eq(pair_id)
                    .and(student_pair::year.eq(year))
            ).get_result(&mut conn)
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
        co_auth_token: &String,
        phase: &String,
    ) -> Result<Auth, Error>;
    async fn get_by_token(
        &self,
        auth_token: &String,
    ) -> Result<Auth, Error>;
    async fn update_phase(
        &self,
        auth_id: &Uuid,
        phase: &String,
    ) -> Result<usize, Error>;
    async fn delete(
        &self,
        auth_ud: &Uuid,
    ) -> Result<usize, Error>;
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
        co_auth_token: &String,
        phase: &String,
    ) -> Result<Auth, Error> {
        let new_auth = NewAuth {
            main_auth_token,
            co_auth_token,
            phase,
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

    async fn update_phase(
            &self,
            auth_id: &Uuid,
            phase: &String,
        ) -> Result<usize, Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::update(auth::table.find(auth_id))
            .set(auth::phase.eq(phase))
            .execute(&mut conn)
    }

    async fn delete(
            &self,
            auth_id: &Uuid
        ) -> Result<usize, Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::delete(auth::table.find(auth_id))
            .execute(&mut conn)
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
        floor: &String,
        prev_status: &String,
        new_status: &String,
    ) -> Result<usize, Error>;

    async fn update_status_by_id(
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

    async fn get_by_floor(
        &self,
        floor: &String,
    ) -> Result<Vec<Locker>, Error>;

    async fn get_by_status(
        &self,
        status: &String,
    ) -> Result<Vec<Locker>, Error>;

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

    async  fn update_status(
            &self,
            floor: &String,
            prev_status: &String,
            new_status: &String,
        ) -> Result<usize, Error> {
        let mut conn = self.pool.get().unwrap();
        let floor_ex = format!("{}%", floor);
        let status_ex = format!("{}%", prev_status);
        diesel::update(
            locker::table.filter(
                locker::locker_id.like(floor_ex)
            ).filter(locker::status.like(status_ex))
        ).set(locker::status.eq(new_status))
        .execute(&mut conn)
    }

    async fn update_status_by_id(
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

    async fn get_by_floor(
            &self,
            floor: &String,
        ) -> Result<Vec<Locker>, Error> {
        let mut conn = self.pool.get().unwrap();
        let floor_ex = format!("{}%", floor);
        locker::table
            .filter(
                locker::locker_id
                .like(floor_ex)
            ).get_results(&mut conn)
    }

    async fn get_by_status(
            &self,
            status: &String,
        ) -> Result<Vec<Locker>, Error> {
        let mut conn = self.pool.get().unwrap();
        locker::table
            .filter(
                locker::status.like(status)
            ).get_results(&mut conn)
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
    ) -> Result<Vec<AssignmentRecord>, Error>;

    async fn get_by_pair_id(
        &self,
        year: &i32,
        pair_id: &Uuid,
    ) -> Result<AssignmentRecord, Error>;

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
    ) -> Result<Vec<AssignmentRecord>, Error> {
        let mut conn = self.pool.get().unwrap();

        let floor_ex = format!("{}%", floor);

        assignment_record::table
        .filter(assignment_record::locker_id
            .like(floor_ex)
        ).filter(assignment_record::pair_id.eq(pair_id).and(assignment_record::year.eq(year))
        ).get_results(&mut conn)
    }

    async  fn get_by_pair_id(
            &self,
            year: &i32,
            pair_id: &Uuid,
        ) -> Result<AssignmentRecord, Error> {
        let mut conn = self.pool.get().unwrap();

        assignment_record::table
            .filter(assignment_record::pair_id.eq(pair_id).and(assignment_record::year.eq(year)))
            .get_result(&mut conn)
    }

    async fn delete_all(
        &self
    ) -> Result<usize, Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::delete(assignment_record::table)
            .execute(&mut conn)
    }
}

// admin

#[async_trait]
pub trait AdminRepository: Send + Sync {
    async fn insert(
        &self,
        username: &String,
        password: &String
    ) -> Result<Admin, Error>;

    async fn get_by_name(
        &self,
        username: &String,
    ) -> Result<Admin, Error>;

    async fn delete_all(
        &self
    ) -> Result<usize, Error>;
}

pub struct AdminRepositorySqlImpl {
    pool : Pool<PgConnection>
}

impl AdminRepositorySqlImpl {
    pub fn new(pool: Pool<PgConnection>) -> Self {
        AdminRepositorySqlImpl { pool }
    }
}

#[async_trait]
impl AdminRepository for AdminRepositorySqlImpl {
    async fn insert (
        &self,
        username: &String,
        password: &String,
    ) -> Result<Admin, Error> {
        let new_admin = NewAdmin {
            username,
            password,
        };
        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(admin::table)
            .values(&new_admin)
            .get_result(&mut conn)
    }

    async fn get_by_name (
            &self,
            username: &String,
    ) -> Result<Admin, Error> {
        let mut conn = self.pool.get().unwrap();
        admin::table.filter(admin::username.eq(username))
            .first(&mut conn)
    }

    async fn delete_all(
        &self
    ) -> Result<usize, Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::delete(admin::table)
            .execute(&mut conn)
    }
}

// locker_auth_info

#[async_trait]
pub trait LockerAuthInfoRepository: Send + Sync {
    async fn insert(
        &self,
        auth_id: &Uuid,
        main_student_id: &String,
        main_family_name: &String,
        main_given_name: &String,
        co_student_id: &String,
        co_family_name: &String,
        co_given_name: &String,
    ) -> Result<LockerAuthInfo, Error>;
    async fn get_by_id(
        &self,
        auth_id: &Uuid,
    ) -> Result<LockerAuthInfo, Error>;
    async fn delete(
        &self,
        auth_id: &Uuid,
    ) -> Result<usize, Error>;
    async fn delete_all(
        &self
    ) -> Result<usize, Error>;
}

pub struct LockerAuthInfoRepositorySqlImpl {
    pool: Pool<PgConnection>
}

impl LockerAuthInfoRepositorySqlImpl {
    pub fn new(pool: Pool<PgConnection>) -> Self {
        LockerAuthInfoRepositorySqlImpl { pool }
    }
}

#[async_trait]
impl LockerAuthInfoRepository for LockerAuthInfoRepositorySqlImpl {
    async fn insert(
            &self,
            auth_id: &Uuid,
            main_student_id: &String,
            main_family_name: &String,
            main_given_name: &String,
            co_student_id: &String,
            co_family_name: &String,
            co_given_name: &String,
    ) -> Result<LockerAuthInfo, Error> {
        let new_auth_info = NewLockerAuthInfo {
            auth_id: auth_id,
            main_student_id: main_student_id,
            main_family_name: main_family_name,
            main_given_name: main_given_name,
            co_student_id: co_student_id,
            co_family_name: co_family_name,
            co_given_name: co_given_name,
        };
        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(locker_auth_info::table)
            .values(&new_auth_info)
            .get_result(&mut conn)
    }
    async fn get_by_id(
            &self,
            auth_id: &Uuid,
        ) -> Result<LockerAuthInfo, Error> {
            let mut conn = self.pool.get().unwrap();
        locker_auth_info::table.filter(locker_auth_info::auth_id.eq(auth_id))
            .get_result(&mut conn)
    }
    async fn delete(
            &self,
            auth_id: &Uuid,
        ) -> Result<usize, Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::delete(locker_auth_info::table.find(auth_id))
            .execute(&mut conn)
    }
    async fn delete_all(
            &self
        ) -> Result<usize, Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::delete(locker_auth_info::table)
            .execute(&mut conn)
    }
}

// circle auth info

#[async_trait]
pub trait CircleAuthInfoRepository: Send + Sync {
    async fn insert(
        &self,
        auth_id: &Uuid,
        main_student_id: &String,
        main_family_name: &String,
        main_given_name: &String,
        main_email: &String,
        main_phone: &String,
        co_student_id: &String,
        co_family_name: &String,
        co_given_name: &String,
        co_email: &String,
        co_phone: &String,
        b_url: &String,
        c_url: &String,
        d_url: &String,
        organization_name: &String,
        organization_ruby: &String,
        organization_email: &String,
    ) -> Result<CircleAuthInfo, Error>;
    async fn get_by_id(
        &self,
        auth_id: &Uuid,
    ) -> Result<CircleAuthInfo, Error>;
    async fn delete(
        &self,
        auth_id: &Uuid,
    ) -> Result<usize, Error>;
    async fn delete_all(
        &self
    ) -> Result<usize, Error>;
}

pub struct CircleAuthInfoRepositorySqlImpl {
    pool: Pool<PgConnection>
}

impl CircleAuthInfoRepositorySqlImpl {
    pub fn new(pool: Pool<PgConnection>) -> Self {
        CircleAuthInfoRepositorySqlImpl { pool }
    }
}

#[async_trait]
impl CircleAuthInfoRepository for CircleAuthInfoRepositorySqlImpl {
    async fn insert(
            &self,
            auth_id: &Uuid,
            main_student_id: &String,
            main_family_name: &String,
            main_given_name: &String,
            main_email: &String,
            main_phone: &String,
            co_student_id: &String,
            co_family_name: &String,
            co_given_name: &String,
            co_email: &String,
            co_phone: &String,
            b_doc: &String,
            c_doc: &String,
            d_doc: &String,
            organization_name: &String,
            organization_ruby: &String,
            organization_email: &String,
    ) -> Result<CircleAuthInfo, Error> {
        let new_auth_info = NewCircleAuthInfo {
            auth_id: auth_id,
            main_student_id: main_student_id,
            main_family_name: main_family_name,
            main_given_name: main_given_name,
            main_email: main_email,
            main_phone: main_phone,
            co_student_id: co_student_id,
            co_family_name: co_family_name,
            co_given_name: co_given_name,
            co_email: co_email,
            co_phone: co_phone,
            b_doc: b_doc,
            c_doc: c_doc,
            d_doc: d_doc,
            organization_name: organization_name,
            organization_ruby: organization_ruby,
            organization_email: organization_email,
        };
        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(circle_auth_info::table)
            .values(&new_auth_info)
            .get_result(&mut conn)
    }
    async fn get_by_id(
            &self,
            auth_id: &Uuid,
        ) -> Result<CircleAuthInfo, Error> {
            let mut conn = self.pool.get().unwrap();
        circle_auth_info::table.filter(circle_auth_info::auth_id.eq(auth_id))
            .get_result(&mut conn)
    }
    async fn delete(
            &self,
            auth_id: &Uuid,
        ) -> Result<usize, Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::delete(circle_auth_info::table.find(auth_id))
            .execute(&mut conn)
    }
    async fn delete_all(
            &self
        ) -> Result<usize, Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::delete(circle_auth_info::table)
            .execute(&mut conn)
    }
}

// representatives

#[async_trait]
pub trait RepresentativesRepository: Send + Sync {
    async fn insert(
        &self,
        student_id: &String,
        family_name: &String,
        given_name: &String,
        email: &String,
        phone: &String,
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

#[async_trait]
impl RepresentativesRepository for RepresentativesRepositorySqlImpl {
    async fn insert(
            &self,
            student_id: &String,
            family_name: &String,
            given_name: &String,
            email: &String,
            phone: &String,
        ) -> Result<Representatives, Error> {
        let new_representative = NewRepresentatives{
            student_id: student_id,
            family_name: family_name,
            given_name: given_name,
            email: email,
            phone: phone,
        };
        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(representatives::table)
            .values(new_representative)
            .on_conflict(representatives::student_id)
            .do_update()
            .set((representatives::updated_at.eq(diesel::dsl::now), representatives::email.eq(email), representatives::phone.eq(phone)))
            .get_result(&mut conn)
    }
}

// registration

#[async_trait]
pub trait RegistrationRepository: Send + Sync {
    async fn insert(
        &self,
        organization_id: &i32,
        year: &i32,
        main_student_id: &String,
        co_student_id: &String,
        status_acceptance: &String,
        status_authentication: &String,
        status_form_confirmation: &String,
        status_registration_complete: &String,
        b_url: &String,
        c_url: &String,
        d_url: &String,
    ) -> Result<Registration, Error>;
}

pub struct RegistrationRepositorySqlImpl {
    pool: Pool<PgConnection>
}

impl RegistrationRepositorySqlImpl {
    pub fn new(pool: Pool<PgConnection>) -> Self {
        RegistrationRepositorySqlImpl { pool }
    }
}

#[async_trait]
impl RegistrationRepository for RegistrationRepositorySqlImpl {
    async fn insert(
            &self,
            organization_id: &i32,
            year: &i32,
            main_student_id: &String,
            co_student_id: &String,
            status_acceptance: &String,
            status_authentication: &String,
            status_form_confirmation: &String,
            status_registration_complete: &String,
            b_doc: &String,
            c_doc: &String,
            d_doc: &String,
        ) -> Result<Registration, Error> {
        let mut conn = self.pool.get().unwrap();
        let new_registration = NewRegistration{
            organization_id: organization_id,
            year: year,
            main_student_id: main_student_id,
            co_student_id: co_student_id,
            status_acceptance: status_acceptance,
            status_authentication:status_authentication,
            status_form_confirmation: status_form_confirmation,
            status_registration_complete: status_registration_complete,
            b_doc: b_doc,
            c_doc: c_doc,
            d_doc: d_doc,
        };
        diesel::insert_into(registration::table)
            .values(new_registration)
            .get_result(&mut conn)
    }
}

/// # organization
#[async_trait]
pub trait OrganizationRepository: Send + Sync {
    async fn insert(
        &self,
        organization_name: &String,
        organization_ruby: &String,
        organization_email: &String,
    ) -> Result<Organization, Error>;

    async fn get_by_id(
        &self,
        organization_id: &i32,
    ) -> Result<Organization, Error>;
}

pub struct OrganizationRepositorySqlImpl {
    pool: Pool<PgConnection>
}

impl OrganizationRepositorySqlImpl {
    pub fn new(pool: Pool<PgConnection>) -> Self {
        OrganizationRepositorySqlImpl { pool }
    }
}

#[async_trait]
impl OrganizationRepository for OrganizationRepositorySqlImpl {
    async fn insert(
            &self,
            organization_name: &String,
            organization_ruby: &String,
            organization_email: &String,
        ) -> Result<Organization, Error> {
        let new_organization = NewOrganization{
            organization_name: organization_name,
            organization_ruby: organization_ruby,
            organization_email: organization_email,
        };
        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(organization::table)
            .values(new_organization)
            .get_result(&mut conn)
    }

    async  fn get_by_id(
            &self,
            organization_id: &i32,
        ) -> Result<Organization, Error> {
        let mut conn = self.pool.get().unwrap();
        organization::table.filter(organization::organization_id.eq(organization_id))
            .get_result(&mut conn)
    }
}
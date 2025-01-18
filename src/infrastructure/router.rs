use std::sync::Arc;
use std::env;
use diesel::{PgConnection, r2d2::ConnectionManager};
use dotenv::dotenv;
use crate::adapters::repository::{
                                admin::AdminRepositorySqlImpl,
                                assignment_record::AssignmentRecordRepositorySqlImpl,
                                auth::AuthRepositorySqlImpl,
                                circle_auth_info::CircleAuthInfoRepositorySqlImpl,
                                locker_auth_info::LockerAuthInfoRepositorySqlImpl,
                                locker::LockerRepositorySqlImpl,
                                organization::OrganizationRepositorySqlImpl,
                                representatives::RepresentativesRepositorySqlImpl,
                                registration::RegistrationRepositorySqlImpl,
                                student_pair::StudentPairRepositorySqlImpl,
                                student::StudentRepositorySqlImpl,
                                time::TimeRepositorySqlImpl,
                            };
use crate::usecase::{
                    student::StudentUsecaseImpl,
                    student_pair::StudentPairUsecaseImpl,
                    assignment_record::AssignmentRecordUsecaseImpl,
                    auth::AuthUsecaseImpl,
                    admin::AdminUsecaseImpl,
                    locker::LockerUsecaseImpl,
                    representatives::RepresentativesUsecaseImpl,
                    organization::OrganizationUsecaseImpl,
                    registration::RegistrationUsecaseImpl,
                    time::TimeUsecaseImpl,
                };

pub type Pool<T> = diesel::r2d2::Pool<ConnectionManager<T>>;
pub struct App{
    pub student: StudentUsecaseImpl,
    pub student_pair: StudentPairUsecaseImpl,
    pub auth: AuthUsecaseImpl,
    pub locker: LockerUsecaseImpl,
    pub assignment_record: AssignmentRecordUsecaseImpl,
    pub admin: AdminUsecaseImpl,
    pub representatives: RepresentativesUsecaseImpl,
    pub organization: OrganizationUsecaseImpl,
    pub registration: RegistrationUsecaseImpl,
    pub time: TimeUsecaseImpl,
}

impl App{
    pub fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
        let manager = ConnectionManager::<PgConnection>::new(&database_url);
        let pool = Pool::builder().build(manager).expect("Failed to create pool");

        let student_repository = StudentUsecaseImpl::new(Arc::new(StudentRepositorySqlImpl::new(pool.clone())));
        let student_pair_repository = StudentPairUsecaseImpl::new(Arc::new(StudentPairRepositorySqlImpl::new(pool.clone())));
        let auth_repository = AuthUsecaseImpl::new(Arc::new(AuthRepositorySqlImpl::new(pool.clone())), Arc::new(LockerAuthInfoRepositorySqlImpl::new(pool.clone())), Arc::new(CircleAuthInfoRepositorySqlImpl::new(pool.clone())));
        let locker_repository = LockerUsecaseImpl::new(Arc::new(LockerRepositorySqlImpl::new(pool.clone())));
        let assignment_record_repository = AssignmentRecordUsecaseImpl::new(Arc::new(AssignmentRecordRepositorySqlImpl::new(pool.clone())));
        let admin_repository = AdminUsecaseImpl::new(Arc::new(AdminRepositorySqlImpl::new(pool.clone())));
        let representatives_repository = RepresentativesUsecaseImpl::new(Arc::new(RepresentativesRepositorySqlImpl::new(pool.clone())));
        let organization_repository = OrganizationUsecaseImpl::new(Arc::new(OrganizationRepositorySqlImpl::new(pool.clone())));
        let registration_repository = RegistrationUsecaseImpl::new(Arc::new(RegistrationRepositorySqlImpl::new(pool.clone())));
        let time_repository = TimeUsecaseImpl::new(Arc::new(TimeRepositorySqlImpl::new(pool.clone())));

        App {
            student: student_repository,
            student_pair: student_pair_repository,
            auth: auth_repository,
            locker: locker_repository,
            assignment_record: assignment_record_repository,
            admin: admin_repository,
            representatives: representatives_repository,
            organization: organization_repository,
            registration: registration_repository,
            time: time_repository,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
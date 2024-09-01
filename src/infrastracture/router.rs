use std::sync::Arc;
use std::env;
use diesel::{PgConnection, r2d2::ConnectionManager};
use dotenv::dotenv;
use crate::adapters::repository::{
                                StudentRepositorySqlImpl,
                                StudentPairRepositorySqlImpl,
                                LockerRepository,
                                LockerRepositorySqlImpl,
                                AuthRepositorySqlImpl,
                                AssignmentRecordRepositorySqlImpl};
use crate::usecase::{
                    student::StudentUsecaseImpl,
                    student_pair::StudentPairUsecaseImpl,
                    assignment_record::AssignmentRecordUsecaseImpl,
                    auth::AuthUsecaseImpl,
                };

pub type Pool<T> = diesel::r2d2::Pool<ConnectionManager<T>>;
pub struct App{
    pub student: StudentUsecaseImpl,
    pub student_pair: StudentPairUsecaseImpl,
    pub auth: AuthUsecaseImpl,
    pub locker: Arc<dyn LockerRepository>,
    pub assignment_record: AssignmentRecordUsecaseImpl
}

impl App{
    pub fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
        let manager = ConnectionManager::<PgConnection>::new(&database_url);
        let pool = Pool::builder().build(manager).expect("Failed to create pool");

        let student_repository = StudentUsecaseImpl::new(Arc::new(StudentRepositorySqlImpl::new(pool.clone())));
        let student_pair_repository = StudentPairUsecaseImpl::new(Arc::new(StudentPairRepositorySqlImpl::new(pool.clone())));
        let auth_repository = AuthUsecaseImpl::new(Arc::new(AuthRepositorySqlImpl::new(pool.clone())));
        let locker_repository = Arc::new(LockerRepositorySqlImpl::new(pool.clone()));
        let assignment_record_repository = AssignmentRecordUsecaseImpl::new(Arc::new(AssignmentRecordRepositorySqlImpl::new(pool.clone())));

        App {
            student: student_repository,
            student_pair: student_pair_repository,
            auth: auth_repository,
            locker: locker_repository,
            assignment_record: assignment_record_repository
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
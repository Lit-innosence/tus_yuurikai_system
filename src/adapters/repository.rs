pub mod student;
pub mod student_pair;
pub mod locker;
pub mod assignment_record;
pub mod admin;
pub mod auth;
pub mod locker_auth_info;
pub mod circle_auth_info;
pub mod registration;
pub mod representatives;
pub mod organization;
pub mod time;

use diesel::result::Error as DieselError;
use diesel::r2d2::PoolError as PoolError;

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("DBConnectionError: {0}")]
    ConnectionError(#[from] PoolError),

    #[error("QueryError: {0}")]
    DieselError(#[from] DieselError),
}
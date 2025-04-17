use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::infrastructure::schema::*;
use crate::infrastructure::models::*;
use crate::infrastructure::router::Pool;
use super::RepositoryError;

/// # admin
pub trait AdminRepository: Send + Sync {
    fn insert(
        &self,
        username: String,
        password: String
    ) -> Result<Admin, RepositoryError>;

    fn get_by_name(
        &self,
        username: String,
    ) -> Result<Admin, RepositoryError>;

    fn delete_by_name(
        &self,
        username: String,
    ) -> Result<usize, RepositoryError>;

    fn delete_all(
        &self
    ) -> Result<usize, RepositoryError>;
}

pub struct AdminRepositorySqlImpl {
    pool : Pool<PgConnection>
}

impl AdminRepositorySqlImpl {
    pub fn new(pool: Pool<PgConnection>) -> Self {
        AdminRepositorySqlImpl { pool }
    }
}

impl AdminRepository for AdminRepositorySqlImpl {
    fn insert (
        &self,
        username: String,
        password: String,
    ) -> Result<Admin, RepositoryError> {
        let new_admin = NewAdmin {
            username: &username,
            password: &password,
        };
        let mut conn = self.pool.get()?;
        let result = diesel::insert_into(admin::table)
            .values(&new_admin)
            .get_result::<Admin>(&mut conn)?;

        Ok(result)
    }

    fn get_by_name (
            &self,
            username: String,
    ) -> Result<Admin, RepositoryError> {
        let mut conn = self.pool.get()?;
        let result = admin::table.filter(admin::username.eq(username))
            .first::<Admin>(&mut conn)?;

        Ok(result)
    }

    fn delete_by_name(
            &self,
            username: String,
        ) -> Result<usize, RepositoryError> {
        let mut conn = self.pool.get()?;
        let result = diesel::delete(admin::table.find(username))
            .execute(&mut conn)?;

        Ok(result)
    }

    fn delete_all(
        &self
    ) -> Result<usize, RepositoryError> {
        let mut conn = self.pool.get()?;
        let result = diesel::delete(admin::table)
            .execute(&mut conn)?;

        Ok(result)
    }
}
use diesel::pg::PgConnection;
use diesel::prelude::*;
use uuid::Uuid;

use crate::infrastructure::schema::*;
use crate::infrastructure::models::*;
use crate::infrastructure::router::Pool;
use super::RepositoryError;

/// # auth
pub trait AuthRepository: Send + Sync {
    fn insert (
        &self,
        main_auth_token: String,
        co_auth_token: String,
        phase: String,
    ) -> Result<Auth, RepositoryError>;

    fn get_by_token(
        &self,
        auth_token: String,
    ) -> Result<Auth, RepositoryError>;

    fn update_phase(
        &self,
        auth_id: Uuid,
        phase: String,
    ) -> Result<usize, RepositoryError>;

    fn delete(
        &self,
        auth_ud: Uuid,
    ) -> Result<usize, RepositoryError>;

    fn delete_all(
        &self
    ) -> Result<usize, RepositoryError>;
}

pub struct AuthRepositorySqlImpl {
    pool: Pool<PgConnection>
}

impl AuthRepositorySqlImpl {
    pub fn new(pool: Pool<PgConnection>) -> Self {
        AuthRepositorySqlImpl { pool }
    }
}

impl AuthRepository for AuthRepositorySqlImpl {
    fn insert(
        &self,
        main_auth_token: String,
        co_auth_token: String,
        phase: String,
    ) -> Result<Auth, RepositoryError> {
        let new_auth = NewAuth {
            main_auth_token: &main_auth_token,
            co_auth_token: &co_auth_token,
            phase: &phase,
        };
        let mut conn = self.pool.get()?;
        let result = diesel::insert_into(auth::table)
            .values(&new_auth)
            .get_result::<Auth>(&mut conn)?;

        Ok(result)
    }

    fn get_by_token(
        &self,
        auth_token: String,
    ) -> Result<Auth, RepositoryError> {
        let mut conn = self.pool.get()?;
        let result = auth::table
            .filter(
                auth::main_auth_token
                    .eq(auth_token.clone())
                    .or(auth::co_auth_token.eq(auth_token)),
            )
            .first::<Auth>(&mut conn)?;

        Ok(result)
    }

    fn update_phase(
            &self,
            auth_id: Uuid,
            phase: String,
        ) -> Result<usize, RepositoryError> {
        let mut conn = self.pool.get()?;
        let result = diesel::update(auth::table.find(auth_id))
            .set(auth::phase.eq(phase))
            .execute(&mut conn)?;

        Ok(result)
    }

    fn delete(
            &self,
            auth_id: Uuid
        ) -> Result<usize, RepositoryError> {
        let mut conn = self.pool.get()?;
        let result = diesel::delete(auth::table.find(auth_id))
            .execute(&mut conn)?;

        Ok(result)
    }

    fn delete_all(
        &self
    ) -> Result<usize, RepositoryError> {
        let mut conn = self.pool.get()?;
        let result = diesel::delete(auth::table)
            .execute(&mut conn)?;

        Ok(result)
    }
}
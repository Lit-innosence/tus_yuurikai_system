use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;

use crate::infrastructure::schema::*;
use crate::infrastructure::models::*;
use crate::infrastructure::router::Pool;

/// # auth
pub trait AuthRepository: Send + Sync {
    fn insert (
        &self,
        main_auth_token: String,
        co_auth_token: String,
        phase: String,
    ) -> Result<Auth, Error>;

    fn get_by_token(
        &self,
        auth_token: String,
    ) -> Result<Auth, Error>;

    fn update_phase(
        &self,
        auth_id: Uuid,
        phase: String,
    ) -> Result<usize, Error>;

    fn delete(
        &self,
        auth_ud: Uuid,
    ) -> Result<usize, Error>;

    fn delete_all(
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

impl AuthRepository for AuthRepositorySqlImpl {
    fn insert(
        &self,
        main_auth_token: String,
        co_auth_token: String,
        phase: String,
    ) -> Result<Auth, Error> {
        let new_auth = NewAuth {
            main_auth_token: &main_auth_token,
            co_auth_token: &co_auth_token,
            phase: &phase,
        };
        let mut conn = self.pool.get().unwrap();
        diesel::insert_into(auth::table)
            .values(&new_auth)
            .get_result(&mut conn)
    }

    fn get_by_token(
        &self,
        auth_token: String,
    ) -> Result<Auth, Error> {
        let mut conn = self.pool.get().unwrap();
        auth::table
            .filter(
                auth::main_auth_token
                    .eq(auth_token.clone())
                    .or(auth::co_auth_token.eq(auth_token)),
            )
            .first(&mut conn)
    }

    fn update_phase(
            &self,
            auth_id: Uuid,
            phase: String,
        ) -> Result<usize, Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::update(auth::table.find(auth_id))
            .set(auth::phase.eq(phase))
            .execute(&mut conn)
    }

    fn delete(
            &self,
            auth_id: Uuid
        ) -> Result<usize, Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::delete(auth::table.find(auth_id))
            .execute(&mut conn)
    }

    fn delete_all(
        &self
    ) -> Result<usize, Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::delete(auth::table)
            .execute(&mut conn)
    }
}
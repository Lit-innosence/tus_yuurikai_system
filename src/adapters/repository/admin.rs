use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use async_trait::async_trait;

use crate::infrastructure::schema::*;
use crate::infrastructure::models::*;
use crate::infrastructure::router::Pool;

/// # admin
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

    async fn delete_by_name(
        &self,
        username: &String,
    ) -> Result<usize, Error>;

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

    async fn delete_by_name(
            &self,
            username: &String,
        ) -> Result<usize, Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::delete(admin::table.find(username))
            .execute(&mut conn)
    }

    async fn delete_all(
        &self
    ) -> Result<usize, Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::delete(admin::table)
            .execute(&mut conn)
    }
}
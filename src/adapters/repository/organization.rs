use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use async_trait::async_trait;

use crate::infrastructure::schema::*;
use crate::infrastructure::models::*;
use crate::infrastructure::router::Pool;

/// # organization
#[async_trait]
pub trait OrganizationRepository: Send + Sync {
    async fn insert(
        &self,
        organization_name: &String,
        organization_ruby: &String,
        organization_email: &String,
    ) -> Result<Organization, Error>;

    async fn get_all(
        &self,
    ) -> Result<Vec<Organization>, Error>;

    async fn get_by_id(
        &self,
        organization_id: &i32,
    ) -> Result<Organization, Error>;

    async fn update_email_by_id(
        &self,
        organization_id: &i32,
        organization_email: &String,
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

    async fn get_all(
            &self,
        ) -> Result<Vec<Organization>, Error> {
        let mut conn = self.pool.get().unwrap();
        organization::table
            .get_results(&mut conn)
    }

    async  fn get_by_id(
            &self,
            organization_id: &i32,
        ) -> Result<Organization, Error> {
        let mut conn = self.pool.get().unwrap();
        organization::table.filter(organization::organization_id.eq(organization_id))
            .get_result(&mut conn)
    }

    async fn update_email_by_id(
            &self,
            organization_id: &i32,
            organization_email: &String,
        ) -> Result<Organization, Error> {
        let mut conn = self.pool.get().unwrap();
        diesel::update(organization::table)
            .filter(organization::organization_id.eq(organization_id))
            .set((organization::organization_email.eq(organization_email), organization::updated_at.eq(diesel::dsl::now)))
            .get_result(&mut conn)
    }
}
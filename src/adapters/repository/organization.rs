use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::infrastructure::schema::*;
use crate::infrastructure::models::*;
use crate::infrastructure::router::Pool;
use super::RepositoryError;

/// # organization
pub trait OrganizationRepository: Send + Sync {
    fn insert(
        &self,
        organization_name: String,
        organization_ruby: String,
        organization_email: String,
    ) -> Result<Organization, RepositoryError>;

    fn get_all(
        &self,
    ) -> Result<Vec<Organization>, RepositoryError>;

    fn get_by_id(
        &self,
        organization_id: i32,
    ) -> Result<Organization, RepositoryError>;

    fn update_email_by_id(
        &self,
        organization_id: i32,
        organization_email: String,
    ) -> Result<Organization, RepositoryError>;
}

pub struct OrganizationRepositorySqlImpl {
    pool: Pool<PgConnection>
}

impl OrganizationRepositorySqlImpl {
    pub fn new(pool: Pool<PgConnection>) -> Self {
        OrganizationRepositorySqlImpl { pool }
    }
}

impl OrganizationRepository for OrganizationRepositorySqlImpl {
    fn insert(
            &self,
            organization_name: String,
            organization_ruby: String,
            organization_email: String,
        ) -> Result<Organization, RepositoryError> {
        let new_organization = NewOrganization{
            organization_name: &organization_name,
            organization_ruby: &organization_ruby,
            organization_email: &organization_email,
        };
        let mut conn = self.pool.get()?;
        let result = diesel::insert_into(organization::table)
            .values(new_organization)
            .get_result::<Organization>(&mut conn)?;

        Ok(result)
    }

    fn get_all(
            &self,
        ) -> Result<Vec<Organization>, RepositoryError> {
        let mut conn = self.pool.get()?;
        let result = organization::table
            .get_results::<Organization>(&mut conn)?;

        Ok(result)
    }

    fn get_by_id(
            &self,
            organization_id: i32,
        ) -> Result<Organization, RepositoryError> {
        let mut conn = self.pool.get()?;
        let result = organization::table.filter(organization::organization_id.eq(organization_id))
            .get_result::<Organization>(&mut conn)?;

        Ok(result)
    }

    fn update_email_by_id(
            &self,
            organization_id: i32,
            organization_email: String,
        ) -> Result<Organization, RepositoryError> {
        let mut conn = self.pool.get()?;
        let result = diesel::update(organization::table)
            .filter(organization::organization_id.eq(organization_id))
            .set((organization::organization_email.eq(organization_email), organization::updated_at.eq(diesel::dsl::now)))
            .get_result::<Organization>(&mut conn)?;

        Ok(result)
    }
}
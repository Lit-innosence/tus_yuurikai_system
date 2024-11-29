use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationInfo {
    #[schema(inline)]
    pub main_user: super::student::RepresentativeInfo,
    #[schema(inline)]
    pub co_user: super::student::RepresentativeInfo,
    #[schema(inline)]
    pub organization: Organization,
    #[schema(example = "https://www.google.com")]
    pub b_url: String,
    #[schema(example = "https://www.google.com")]
    pub c_url: String,
    #[schema(example = "https://www.google.com")]
    pub d_url: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationUpdateInfo {
    #[schema(example = "C00000")]
    pub organization_id: String,
    #[schema(inline)]
    pub main_user: super::student::RepresentativeInfo,
    #[schema(inline)]
    pub co_user: super::student::RepresentativeInfo,
    #[schema(example = "example@example.com")]
    pub organization_email: String,
    #[schema(example = "https://www.google.com")]
    pub b_url: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    #[schema(example = "ロケット団")]
    pub organization_name: String,
    #[schema(example = "ろけっとだん")]
    pub organization_ruby: String,
    #[schema(example = "rokect@example.com")]
    pub organization_email: String,
}
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationInfo {
    #[schema(inline)]
    pub main_user: super::student::RepresentativeInfo,
    #[schema(inline)]
    pub co_user: super::student::RepresentativeInfo,
    #[schema(inline)]
    pub organization: Organization,
    #[schema(example = "xxxxxxxxxxxxxxxxxxxxxx")]
    pub b_doc: String,
    #[schema(example = "xxxxxxxxxxxxxxxxxxxxxx")]
    pub c_doc: String,
    #[schema(example = "xxxxxxxxxxxxxxxxxxxxxx")]
    pub d_doc: String,
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
    #[schema(example = "xxxxxxxxxxxxxxxxxxxxxx")]
    pub b_doc: String,
}

#[derive(Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    #[schema(example = "ロケット団")]
    pub organization_name: String,
    #[schema(example = "ろけっとだん")]
    pub organization_ruby: String,
    #[schema(example = "rokect@example.com")]
    pub organization_email: String,
}
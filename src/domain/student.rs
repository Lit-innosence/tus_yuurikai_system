use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    #[schema(example = "4622999")]
    pub student_id: String,
    #[schema(example = "山田")]
    pub family_name: String,
    #[schema(example = "太郎")]
    pub given_name: String,
}

#[derive(Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RepresentativeInfo {
    #[schema(example = "4622999")]
    pub student_id: String,
    #[schema(example = "山田")]
    pub family_name: String,
    #[schema(example = "太郎")]
    pub given_name: String,
    #[schema(example = "example@example.com")]
    pub email: String,
    #[schema(example = "000-0000-0000")]
    pub phone_number: String,
}
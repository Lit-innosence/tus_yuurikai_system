use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PairInfo {
    pub main_user: super::student::UserInfo,
    pub co_user: super::student::UserInfo,
}

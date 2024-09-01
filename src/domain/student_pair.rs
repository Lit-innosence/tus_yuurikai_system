use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PairInfo {
    #[schema(inline)]
    pub main_user: super::student::UserInfo,
    #[schema(inline)]
    pub co_user: super::student::UserInfo,
}

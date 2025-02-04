use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AssignmentInfo {
    #[schema(example = "4622999")]
    pub student_id: String,
    #[schema(example = "2001")]
    pub locker_id: String,
}

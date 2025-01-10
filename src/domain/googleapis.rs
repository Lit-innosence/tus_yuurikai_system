use crate::domain::circle::OrganizationInfo;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct MakeContactParameters {
    pub data: OrganizationInfo,
}

#[derive(Serialize, Deserialize)]
pub struct MakeContact {
    pub function: String,
    pub parameters: MakeContactParameters,
}
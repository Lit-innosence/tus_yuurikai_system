pub mod circle;
pub mod locker;

use crate::adapters::httpmodels::HealthCheckRequest;
use crate::adapters::controller::{locker::*, circle::*};
use crate::adapters::httpmodels::*;
use crate::domain::{student::{UserInfo, RepresentativeInfo}, student_pair::PairInfo, assignment::AssignmentInfo, circle::{OrganizationInfo, Organization, OrganizationUpdateInfo}};
use rocket::{get, post, serde::json::Json};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        get_healthcheck,
        post_healthcheck,
        token_generator,
        main_auth,
        co_auth,
        auth_check,
        locker_register,
        login,
        user_search,
        availability,
        reset,
        update_entry,
        update_token_generator,
        register_token_generator,
        circle_main_auth,
        circle_co_auth,
    ),
    components(schemas(
        HealthCheckRequest,
        UserInfo,
        PairInfo,
        LockerTokenGenRequest,
        AuthCheckResponse,
        AssignmentInfo,
        LockerResisterRequest,
        LoginFormRequest,
        LockerStatusResponse,
        UserSearchResponse,
        LockerResetRequest,
        RepresentativeInfo,
        Organization,
        OrganizationInfo,
        OrganizationUpdateInfo,
        CircleUpdateRequest,
        CircleTokenGenRequest,
        CircleUpdateTokenGenRequest,
    ))
)]
pub struct ApiDoc;

// GETヘルスチェック
#[utoipa::path(context_path = "/api")]
#[get("/get-healthcheck")]
pub fn get_healthcheck() -> &'static str {
    "Hello, world!"
}

// POSTヘルスチェック
#[utoipa::path(context_path = "/api")]
#[post("/post-healthcheck", data = "<data>")]
pub fn post_healthcheck(data: Json<HealthCheckRequest>) -> String {
    format!("Accepted post request! {:?}", data.text)
}
use crate::adapters::httpmodels::*;
use crate::infrastructure::{router::App, models::{AssignmentRecord, StudentPair}};
use crate::usecase::{
                    student::StudentUsecase,
                    student_pair::StudentPairUsecase,
                    assignment_record::AssignmentRecordUsecase,
                    auth::AuthUsecase,
                    locker::LockerUsecase,
                    admin::AdminUsecase};
use crate::utils::{decode_jwt::decode_jwt, encode_jwt::encode_jwt};

use std::{env, collections::HashSet};
use dotenv::dotenv;
use rocket::{get, http::{Status, RawStr, Cookie, CookieJar, SameSite}, post, serde::json::Json, State};
use chrono::Duration;

// 団体情報更新API
#[utoipa::path(context_path = "/api/circle")]
#[post("/update", data = "<request>")]
pub async fn circle_update(request: Json<CircleUpdateRequest>, app: &State<App>) -> (Status, &'static str) {
    (Status::Ok, "organization updated successfully")
}
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
use utoipa::openapi::request_body;

// 団体にかかわるAPIをここに追加します
#[utoipa::path(context_path = "/api/circle")]
#[post("/update/entry", data = "<request>")]
pub async fn update_entry(request: Json<CircleUpdateRequest>, app: &State<App>) -> (Status, &'static str) {
    // 環境変数からURLを取得
    dotenv().ok();
    let app_url = env::var("GFORM_UPDATE_URL").expect("GFORM_UPDATE_URL must be set");

    // メール内容の作成
    let user_address = format!("{}", request.email);
    let content = format!("{} 代表 {} {} 様\n\n以下のURLから団体情報更新用GoogleFormにアクセスして更新内容を入力してください。\n\n{}", request.organization_name, request.family_name, request.given_name, app_url);
    let subject = "団体情報更新 更新用URLのお知らせ";

    // 認証メールを送信
    if app.auth.mail_sender(user_address, content, subject).await.is_err(){
        return (Status::InternalServerError, "Failed to send authentication email");
    }

    // レスポンスを返す
    (Status::Ok, "Authentication email sent successfully")
}
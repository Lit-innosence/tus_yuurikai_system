use crate::adapters::httpmodels::*;
use crate::domain::circle::{OrganizationInfo, Organization};
use crate::infrastructure::router::App;
use crate::usecase::{
                    auth::AuthUsecase,
                    admin::AdminUsecase};
use crate::utils::{decode_jwt::decode_jwt, encode_jwt::encode_jwt};

use std::env;
use dotenv::dotenv;
use rocket::{get, http::{Status, RawStr, Cookie, CookieJar, SameSite}, post, serde::json::Json, State};
use chrono::Duration;
use utoipa::openapi::request_body;

// 団体登録受付API
#[utoipa::path(context_path = "/api/circle")]
#[post("/update/entry", data = "<request>")]
pub async fn update_entry(request: Json<CircleUpdateRequest>, app: &State<App>) -> (Status, &'static str) {
    // 環境変数からURLを取得
    dotenv().ok();
    let app_url = env::var("GFORM_UPDATE_URL").expect("GFORM_UPDATE_URL must be set");

    // 団体が存在しているかの確認

    // メール内容の作成
    let user_address = format!("{}", request.email);
    let content = format!("{} 代表 {}{} 様\n\n以下のURLから団体情報更新用GoogleFormにアクセスして更新内容を入力してください。\n\n{}", request.organization_name, request.family_name, request.given_name, app_url);
    let subject = "団体情報更新 更新用URLのお知らせ";

    // 認証メールを送信
    if app.auth.mail_sender(user_address, content, subject).await.is_err(){
        return (Status::InternalServerError, "Failed to send authentication email");
    }

    // レスポンスを返す
    (Status::Ok, "Authentication email sent successfully")
}

// 団体情報更新認証API
#[utoipa::path(context_path = "/api/circle")]
#[post("/update/token-gen", data= "<request>")]
pub async fn update_token_generator(request: Json<CircleUpdateTokenGenRequest>, app: &State<App>) -> (Status, &'static str) {

    // リクエストからデータを取得
    let data = &request.data;

    // OrganizationInfoに成形
    let auth_info = OrganizationInfo {
        main_user: data.main_user.clone(),
        co_user: data.co_user.clone(),
        organization: Organization {
            organization_name: String::from(""),
            organization_ruby: String::from(""),
            organization_email: data.organization_email.clone(),
        },
        b_url: data.b_url.clone(),
        c_url: String::from(""),
        d_url: String::from(""),
    };

    // 団体情報をDBに登録し、auth_tokenを取得
    let token = match app.auth.circle_register(&auth_info, &String::from("main_auth"), false).await {
        Ok(auth) => auth.main_auth_token,
        Err(_) => {return (Status::InternalServerError, "failed to issue auth token")}
    };

    // メール内容の作成
    let main_user = &data.main_user;

    dotenv().ok();
    let app_url = env::var("APP_URL").expect("APP_URL must be set.");

    let user_address = main_user.email.to_string();
    let content = format!("{}{} 様\n\n以下のURLにアクセスして認証を完了してください。\n{}/circle/update/auth?method=1&token={}&id={}", main_user.family_name, main_user.given_name, app_url, token, data.organization_id);
    let subject = "ロッカーシステム メール認証";

    // メールの送信
    if app.auth.mail_sender(user_address, content, subject).await.is_err(){
        return (Status::InternalServerError, "failed to send authentication email");
    }

    (Status::Created, "Authentication email sent successfully")
}

// 団体登録認証API
#[utoipa::path(context_path = "/api/circle")]
#[post("/register/token-gen", data="<request>")]
pub async fn register_token_generator(request: Json<CircleTokenGenRequest>, app: &State<App>) -> (Status, &'static str) {

    // リクエストからデータを取得
    let data = &request.data;

    // 団体情報をDBに登録し、auth_tokenを取得
    let token = match app.auth.circle_register(data, &String::from("main_auth"), false).await {
        Ok(auth) => auth.main_auth_token,
        Err(_) => {return (Status::InternalServerError, "failed to issue auth token")}
    };

    // メール内容の作成
    let main_user = &data.main_user;

    dotenv().ok();
    let app_url = env::var("APP_URL").expect("APP_URL must be set.");

    let user_address = main_user.email.to_string();
    let content = format!("{}{} 様\n\n以下のURLにアクセスして認証を完了してください。\n{}/circle/register/auth?method=1&token={}", main_user.family_name, main_user.given_name, app_url, token);
    let subject = "ロッカーシステム メール認証";

    // メールの送信
    if app.auth.mail_sender(user_address, content, subject).await.is_err() {
        return (Status::InternalServerError, "failed to send authentication email");
    }

    (Status::Created, "Authentication email sent successfully")
}

// 団体代表者認証API

// 団体副代表者認証API
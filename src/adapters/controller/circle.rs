use crate::adapters::httpmodels::*;
use crate::domain::{circle::{OrganizationInfo, Organization}, student::RepresentativeInfo};
use crate::infrastructure::router::App;
use crate::usecase::representatives::RepresentativesUsecase;
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
        b_doc: data.b_doc.clone(),
        c_doc: String::from(""),
        d_doc: String::from(""),
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
    let subject = "団体登録システム メール認証";

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
    let subject = "団体登録システム メール認証";

    // メールの送信
    if app.auth.mail_sender(user_address, content, subject).await.is_err() {
        return (Status::InternalServerError, "failed to send authentication email");
    }

    (Status::Created, "Authentication email sent successfully")
}

// 団体代表者認証API
#[utoipa::path(context_path = "/api/circle")]
#[post("/main-auth?<token>&<id>")]
pub async fn circle_main_auth(token: String, id: Option<String>, app:&State<App>) -> (Status, &'static str) {

    // tokenが一致するレコードを取得
    let auth = match app.auth.token_check(token, true).await{
        Ok(auth) => auth,
        // 存在しなかったら終了
        Err(status) => return (status, "invalid token"),
    };

    // authのphaseを確認
    if auth.phase != String::from("main_auth") {
        return (Status::BadRequest, "authentication phase does not match");
    }

    // auth_infoからレコードを取得
    let auth_info = match app.auth.get_circle_auth_info(&auth.auth_id).await {
        Ok(info) => info,
        Err(status) => return (status, "failed to get circle info"),
    };

    // main_userの情報を格納
    let main_user = &RepresentativeInfo{
        student_id: auth_info.main_student_id,
        family_name: auth_info.main_family_name,
        given_name: auth_info.main_given_name,
        email: auth_info.main_email,
        phone_number: auth_info.main_phone,
    };

    if app.representatives.register(main_user).await.is_err() {
        return (Status::InternalServerError, "failed to insert Representative")
    }

    let co_user = &RepresentativeInfo{
        student_id: auth_info.co_student_id,
        family_name: auth_info.co_family_name,
        given_name: auth_info.co_given_name,
        email: auth_info.co_email,
        phone_number: auth_info.co_phone,
    };

    dotenv().ok();
    let app_url = env::var("APP_URL").expect("APP_URL must be set.");

    let user_address = co_user.email.to_string();
    let content = match id {
        Some(id) => format!("{}{} 様\n\n以下のURLにアクセスして認証を完了してください。\n{}/circle/update/auth?method=0&token={}&id={}", co_user.family_name, co_user.given_name, app_url, auth.co_auth_token, id),
        None => format!("{}{} 様\n\n以下のURLにアクセスして認証を完了してください。\n{}/circle/register/auth?method=0&token={}", co_user.family_name, co_user.given_name, app_url, auth.co_auth_token),
    };
    let subject = "団体登録システム メール認証";

    // メールの送信
    if app.auth.mail_sender(user_address, content, subject).await.is_err() {
        return (Status::InternalServerError, "failed to send authentication email");
    }

    // phaseの更新
    if app.auth.update_phase(&auth.auth_id, String::from("co_auth")).await.is_err() {
        return (Status::InternalServerError, "failed to update phase");
    }

    (Status::Created, "Authentication email sent successfully")
}

// 団体副代表者認証API
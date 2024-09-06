use crate::domain::{student::UserInfo, student_pair::PairInfo, assignment::AssignmentInfo};
use crate::infrastracture::router::App;
use crate::usecase::{
                    student::StudentUsecase,
                    student_pair::StudentPairUsecase,
                    assignment_record::AssignmentRecordUsecase,
                    auth::AuthUsecase};

use std::env;
use dotenv::dotenv;
use rocket::{get, http::Status, post, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

#[derive(OpenApi)]
#[openapi(
    paths(
        get_healthcheck,
        post_healthcheck,
        token_generator,
        main_auth,
        co_auth,
        locker_register,
    ),
    components(schemas(
        HealthCheckRequest,
        UserInfo,
        PairInfo,
        TokenGenRequest,
        AssignmentInfo,
        LockerResisterRequest,
    ))
)]
pub struct ApiDoc;

// GETヘルスチェック
#[utoipa::path(context_path = "")]
#[get("/get_healthcheck")]
pub fn get_healthcheck() -> &'static str {
    "Hello, world!"
}

// POSTヘルスチェック
#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckRequest {
    #[schema(example = "Hello world from json!")]
    pub text: String,
}

#[utoipa::path(context_path = "")]
#[post("/post_healthcheck", data = "<data>")]
pub fn post_healthcheck(data: Json<HealthCheckRequest>) -> String {
    format!("Accepted post request! {:?}", data.text)
}

// token生成、メール送信API
#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TokenGenRequest {
    pub data: PairInfo,
}
#[utoipa::path(context_path = "")]
#[post("/locker/token-gen", data = "<request>")]
pub async fn token_generator(request: Json<TokenGenRequest>, app: &State<App>) -> Status {

    let data = &request.data;

    let token = match app.auth.register(&data.main_user.clone(), &data.co_user.clone()).await{
        Ok(auth) => auth.main_auth_token,
        Err(_) => return Status::InternalServerError,
    };

    // メール内容の作成
    let main_user = &data.main_user;

    dotenv().ok();
    let app_url = env::var("APP_URL").expect("APP_URL must be set.");

    let user_address = format!("{}@ed.tus.ac.jp", main_user.student_id);
    let content = format!("{}{} 様\n\n以下のURLにアクセスして認証を完了してください。\n{}/locker/user-register?token={}", main_user.family_name, main_user.given_name, app_url, token);
    let subject = "ロッカーシステム メール認証";

    if app.auth.mail_sender(user_address, content, subject).await.is_err(){
        return Status::InternalServerError;
    }

    Status::Created
}

// main_user認証API {
#[utoipa::path(context_path = "")]
#[get("/locker/main-auth?<token>")]
pub async fn main_auth(token: String, app: &State<App>) -> Status {

    let auth = match app.auth.token_check(token, true).await{
        Ok(auth) => auth,
        Err(status) => return status,
    };

    let main_user = &UserInfo{
        student_id: auth.main_student_id.clone(),
        family_name: auth.main_family_name.clone(),
        given_name: auth.main_given_name.clone(),
    };

    if app.student.register(&main_user.clone()).await.is_err(){
        return Status::InternalServerError;
    }

    let co_user = &UserInfo{
        student_id: auth.co_student_id.clone(),
        family_name: auth.co_family_name.clone(),
        given_name: auth.co_given_name.clone(),
    };

    // メール内容の作成
    dotenv().ok();
    let app_url = env::var("APP_URL").expect("APP_URL must be set.");

    let user_address = format!("{}@ed.tus.ac.jp", co_user.student_id);
    let content = format!("{}{} 様\n\n以下のURLにアクセスして認証を完了してください。\n{}/locker/user-register?token={}", co_user.family_name, co_user.given_name, app_url, auth.co_auth_token);
    let subject = "ロッカーシステム メール認証";

    if app.auth.mail_sender(user_address, content, subject).await.is_err(){
        return Status::InternalServerError;
    }

    Status::Created
}

// co_user認証API {
#[utoipa::path(context_path = "")]
#[get("/locker/co-auth?<token>")]
pub async fn co_auth(token: String, app: &State<App>) -> Status {

    let auth = match app.auth.token_check(token, false).await{
        Ok(auth) => auth,
        Err(status) => return status,
    };

    let co_user = &UserInfo{
        student_id: auth.co_student_id.clone(),
        family_name: auth.co_family_name.clone(),
        given_name: auth.co_given_name.clone(),
    };

    if app.student.register(&co_user.clone()).await.is_err(){
        return Status::InternalServerError;
    }

    let main_user = &UserInfo{
        student_id: auth.main_student_id.clone(),
        family_name: auth.main_family_name.clone(),
        given_name: auth.main_given_name.clone(),
    };

    let student_pair = &PairInfo{
        main_user: main_user.clone(),
        co_user: co_user.clone(),
    };

    if app.student_pair.register(student_pair).await.is_err(){
        return Status::InternalServerError;
    }

    dotenv().ok();
    let app_url = env::var("APP_URL").expect("APP_URL must be set.");

    // リンクが未定義
    let user_address = format!("{}@ed.tus.ac.jp", main_user.student_id);
    let content = format!("認証が完了しました。\n以下のリンクから使用するロッカー番号を選択してください。\n\n{}", app_url);
    let subject = "ロッカーシステム 認証完了通知";

    if app.auth.mail_sender(user_address, content, subject).await.is_err(){
        return Status::InternalServerError;
    }

    Status::Created
}

// ロッカー空き状態確認API

// ロッカー登録API
#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LockerResisterRequest {
    pub data: AssignmentInfo,
}

#[utoipa::path(context_path = "")]
#[post("/locker/locker-register", data = "<request>")]
pub async fn locker_register(request: Json<LockerResisterRequest>, app: &State<App>) -> (Status, &'static str) {

    let assignment = &request.data;

    // pair_idの検索
    let user_pair = match app.student_pair.get_by_id(assignment).await {
        Ok(student_pair) => student_pair,
        Err(_) => return (Status::InternalServerError, "failed to get student_pair id"),
    };

    // 割り当て情報の登録
    if app.assignment_record.register(&user_pair, assignment).await.is_err() {
        return (Status::InternalServerError, "failed to insert request");
    }

    (Status::Created, "success create assignment")
}

// 完了通知API

// パスワード照合API

// ロッカー利用者検索API
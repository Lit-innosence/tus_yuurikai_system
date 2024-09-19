use crate::domain::{student::UserInfo, student_pair::PairInfo, assignment::AssignmentInfo};
use crate::infrastracture::router::App;
use crate::usecase::{
                    student::StudentUsecase,
                    student_pair::StudentPairUsecase,
                    assignment_record::AssignmentRecordUsecase,
                    auth::AuthUsecase};

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
#[get("/get-healthcheck")]
pub fn get_healthcheck() -> &'static str {
    "Hello, world!"
}

// POSTヘルスチェック
#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckRequest {
    #[schema(example = "Hello world from json!")]
    pub text: String,
}

#[utoipa::path(context_path = "")]
#[post("/post-healthcheck", data = "<data>")]
pub fn post_healthcheck(data: Json<HealthCheckRequest>) -> String {
    format!("Accepted post request! {:?}", data.text)
}

// token生成、メール送信API
#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TokenGenRequest {
    pub data: PairInfo,
}
#[utoipa::path(context_path = "/locker")]
#[post("/token-gen", data = "<request>")]
pub async fn token_generator(request: Json<TokenGenRequest>, app: &State<App>) -> Status {

    let data = &request.data;

    let token = match app.auth.register(&data.main_user.clone(), &data.co_user.clone()).await{
        Ok(auth) => auth.main_auth_token,
        Err(_) => return Status::InternalServerError,
    };

    if app.auth.mail_sender(&data.main_user.clone(), token).await.is_err(){
        return Status::InternalServerError;
    }

    Status::Created
}

// main_user認証API {
#[utoipa::path(context_path = "/locker")]
#[get("/main-auth?<token>")]
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

    if app.auth.mail_sender(&co_user.clone(), auth.co_auth_token).await.is_err(){
        return Status::InternalServerError;
    }

    Status::Created
}

// co_user認証API {
#[utoipa::path(context_path = "/locker")]
#[get("/co-auth?<token>")]
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

    Status::Created
}

// ロッカー空き状態確認API

// ロッカー登録API
#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LockerResisterRequest {
    pub data: AssignmentInfo,
}

#[utoipa::path(context_path = "/locker")]
#[post("/locker-register", data = "<request>")]
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
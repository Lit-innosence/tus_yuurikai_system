use crate::domain::{student::UserInfo, student_pair::PairInfo, assignment::AssignmentInfo};
use crate::infrastracture::router::App;
use crate::usecase::{
                    student::StudentUsecase,
                    student_pair::StudentPairUsecase,
                    assignment_record::AssignmentRecordUsecase,
                    auth::AuthUsecase};

use rand::{distributions::Alphanumeric, Rng};
use rocket::{get, http::Status, post, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

#[derive(OpenApi)]
#[openapi(
    paths(
        get_healthcheck,
        post_healthcheck,
        mail_auth,
        user_register,
        locker_register,
    ),
    components(schemas(
        HealthCheckRequest,
        UserInfo,
        PairInfo,
        UserRegisterRequest,
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

// 認証メール送信API
#[utoipa::path(context_path = "")]
#[post("/locker/mail-sender", data = "<request>")]
pub async fn mail_auth(request: Json<UserRegisterRequest>, app: &State<App>) -> Status {

    let data = &request.data;

    if app.auth.mail_sender(&data.main_user.clone()).await.is_err(){
        return Status::InternalServerError;
    }

    if app.auth.mail_sender(&data.co_user.clone()).await.is_err(){
        return Status::InternalServerError;
    }

    Status::Created
}

// ユーザー情報登録API
#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserRegisterRequest {
    pub data: PairInfo,
}

#[utoipa::path(context_path = "")]
#[post("/locker/user-register", data = "<request>")]
pub async fn user_register(request: Json<UserRegisterRequest>, app: &State<App>) -> Status {
    // メインユーザーの登録
    let main_user = &request.data.main_user;

    if app.student.register(&main_user.clone()).await.is_err(){
        return Status::InternalServerError;
    }

    // 共用ユーザーの登録
    let co_user = &request.data.co_user;
    if app.student.register(&co_user.clone()).await.is_err(){
        return Status::InternalServerError;
    }

    // ペア情報の登録
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
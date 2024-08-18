mod db_connector;
mod models;
mod schema;
use rocket::{Build, Rocket, get, post, routes,
            serde::json::Json, http::Status};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;
use serde::{Serialize, Deserialize};
use chrono::{Datelike, Local};

#[derive(OpenApi)]
#[openapi(
    paths(
        get_healthcheck,
        post_healthcheck,
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
struct ApiDoc;

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckRequest {
    #[schema(example = "Hello world from json!")]
    pub text: String
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserRegisterRequest {
    pub data: PairInfo,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PairInfo {
    pub main_user: UserInfo,
    pub co_user: UserInfo,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    #[schema(example = "4622999")]
    pub student_id: String,
    #[schema(example = "山田")]
    pub family_name: String,
    #[schema(example = "太郎")]
    pub given_name: String,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LockerResisterRequest {
    pub data: AssignmentInfo,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AssignmentInfo {
    #[schema(example = "4622999")]
    pub student_id: String,
    #[schema(example = "2001")]
    pub locker_id: i32,
}

// GETヘルスチェック
#[utoipa::path(context_path = "")]
#[get("/get_healthcheck")]
fn get_healthcheck() -> &'static str {
    "Hello, world!"
}

// POSTヘルスチェック
#[utoipa::path(context_path = "")]
#[post("/post_healthcheck", data = "<data>")]
fn post_healthcheck(data: Json<HealthCheckRequest>) -> String {
    format!("Accepted post request! {:?}", data.text)
}

// ユーザー情報登録API
#[utoipa::path(context_path = "")]
#[post("/locker/user-register", data = "<request>")]
fn user_register(request: Json<UserRegisterRequest>) -> Status {
    // データベース接続
    let mut conn = match db_connector::create_connection() {
        Ok(connection) => connection,
        Err(_) => return Status::InternalServerError,
    };

    // メインユーザーの登録
    let main_user = &request.data.main_user;
    if db_connector::insert_student(&mut conn, &main_user.student_id, &main_user.family_name, &main_user.given_name).is_err() {
        return Status::InternalServerError;
    }

    // 共用ユーザーの登録
    let co_user = &request.data.co_user;
    if db_connector::insert_student(&mut conn, &co_user.student_id, &co_user.family_name, &co_user.given_name).is_err() {
        return Status::InternalServerError;
    }

    // ペア情報の登録
    let year = Local::now().year();
    if db_connector::insert_studentpair(&mut conn, &main_user.student_id, &co_user.student_id, &year).is_err() {
        return Status::InternalServerError;
    }

    Status::Created
}

// ロッカー空き状態確認API

// ロッカー登録API
#[utoipa::path(context_path = "")]
#[post("/locker/locker-register", data = "<request>")]
fn locker_register(request: Json<LockerResisterRequest>) -> String {
    let assignmentinfo = &request.data;

    // pair_idの検索

    // データベース登録

    // 割り当て情報の登録

    format!("Request accepted! {:?} {:?}", assignmentinfo.student_id, assignmentinfo.locker_id)
}

// メール認証API

// 完了通知API

// パスワード照合API

// ロッカー利用者検索API

#[rocket::launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![get_healthcheck, post_healthcheck, user_register, locker_register])
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>")
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
}


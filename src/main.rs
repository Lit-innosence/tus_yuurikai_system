mod db_connector;
mod models;
mod schema;
use chrono::{Datelike, Local};
use diesel::dsl::family;
use dotenv::dotenv;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rand::{distributions::Alphanumeric, Rng};
use rocket::{get, http::Status, post, routes, serde::json::Json, Build, Rocket};
use serde::{Deserialize, Serialize};
use std::env;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

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
struct ApiDoc;

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckRequest {
    #[schema(example = "Hello world from json!")]
    pub text: String,
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
    pub locker_id: String,
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

// 認証メール送信API
#[utoipa::path(context_path = "")]
#[post("/locker/mail-sender", data = "<request>")]
fn mail_auth(request: Json<UserRegisterRequest>) -> Status {
    // 環境変数の読み取り
    dotenv().ok();

    let sender_address = env::var("SENDER_MAIL_ADDRESS").expect("SENDER_MAIL_ADDRESS must be set.");

    let appkey = env::var("MAIL_APP_KEY").expect("MAIL_APP_KEY must be set.");

    let app_url = env::var("APP_URL").expect("APP_URL must be set.");

    // メール送信
    if mail_sender(&sender_address, &appkey, &app_url, &request.data.main_user).is_err() {
        return Status::InternalServerError;
    }

    if mail_sender(&sender_address, &appkey, &app_url, &request.data.co_user).is_err() {
        return Status::InternalServerError;
    }

    Status::Created
}

fn mail_sender(
    sender_address: &String,
    appkey: &String,
    app_url: &String,
    user: &UserInfo,
) -> Result<(), Status> {
    // トークンの生成
    let mut rng = rand::thread_rng();
    let token: String = (0..16).map(|_| rng.sample(Alphanumeric) as char).collect();

    // 一時保存データベースにトークンと学生情報を保存
    let mut conn = db_connector::create_connection().map_err(|_| Status::InternalServerError)?;

    if db_connector::insert_auth(
        &mut conn,
        &token,
        &user.student_id,
        &user.family_name,
        &user.given_name,
    )
    .is_err()
    {
        return Err(Status::InternalServerError);
    }

    // メール内容の作成
    let user_address = format!("{}@ed.tus.ac.jp", user.student_id);

    let content = format!("{}{} 様\n\n以下のURLにアクセスして認証を完了してください。\n{}/locker/user-register?token={}",user.family_name, user.given_name, app_url, token);

    let email = Message::builder()
        .from(
            format!("Developer <{}>", sender_address)
                .parse()
                .map_err(|_| Status::InternalServerError)?,
        )
        .to(format!("User <{}>", user_address)
            .parse()
            .map_err(|_| Status::InternalServerError)?)
        .subject("ロッカーシステム メール認証")
        .header(ContentType::TEXT_PLAIN)
        .body(content)
        .map_err(|_| Status::InternalServerError)?;

    let creds = Credentials::new(sender_address.to_owned(), appkey.to_owned());

    // Gmailにsmtp接続する
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .map_err(|_| Status::InternalServerError)?
        .credentials(creds)
        .build();

    // メール送信
    mailer
        .send(&email)
        .map_err(|_| Status::InternalServerError)?;

    Ok(())
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
    if db_connector::insert_student(
        &mut conn,
        &main_user.student_id,
        &main_user.family_name,
        &main_user.given_name,
    )
    .is_err()
    {
        return Status::InternalServerError;
    }

    // 共用ユーザーの登録
    let co_user = &request.data.co_user;
    if db_connector::insert_student(
        &mut conn,
        &co_user.student_id,
        &co_user.family_name,
        &co_user.given_name,
    )
    .is_err()
    {
        return Status::InternalServerError;
    }

    // ペア情報の登録
    let year = Local::now().year();
    if db_connector::insert_studentpair(
        &mut conn,
        &main_user.student_id,
        &co_user.student_id,
        &year,
    )
    .is_err()
    {
        return Status::InternalServerError;
    }

    Status::Created
}

// ロッカー空き状態確認API

// ロッカー登録API
#[utoipa::path(context_path = "")]
#[post("/locker/locker-register", data = "<request>")]
fn locker_register(request: Json<LockerResisterRequest>) -> (Status, &'static str) {
    // データベース接続
    let mut conn = match db_connector::create_connection() {
        Ok(connection) => connection,
        Err(_) => return (Status::InternalServerError, "failed to connect database"),
    };

    let assignment = &request.data;
    let year = Local::now().year();

    // pair_idの検索
    let user_pair = match db_connector::get_studentpair_by_student_id_and_year(
        &mut conn,
        &assignment.student_id,
        &year,
    ) {
        Ok(student_pair) => student_pair,
        Err(_) => return (Status::InternalServerError, "failed to get student_pair id"),
    };

    // 割り当て情報の登録
    if db_connector::insert_assignmentrecord(
        &mut conn,
        &user_pair.pair_id,
        &assignment.locker_id,
        &year,
    )
    .is_err()
    {
        return (Status::InternalServerError, "failed to insert request");
    }

    (Status::Created, "success create assignment")
}

// 完了通知API

// パスワード照合API

// ロッカー利用者検索API

#[rocket::launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount(
            "/",
            routes![
                get_healthcheck,
                post_healthcheck,
                mail_auth,
                user_register,
                locker_register
            ],
        )
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
}

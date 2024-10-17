use crate::domain::{student::UserInfo, student_pair::PairInfo, assignment::AssignmentInfo};
use crate::infrastructure::{router::App, models::{AssignmentRecord, StudentPair}};
use crate::usecase::{
                    student::StudentUsecase,
                    student_pair::StudentPairUsecase,
                    assignment_record::AssignmentRecordUsecase,
                    auth::AuthUsecase,
                    locker::LockerUsecase,
                    admin::AdminUsecase};
use crate::utils::decode_jwt::decode_jwt;

use std::{env, collections::HashSet};
use dotenv::dotenv;
use rocket::{get, http::{Status, RawStr, Cookie, CookieJar}, post, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, Header, Algorithm, EncodingKey};

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
    ),
    components(schemas(
        HealthCheckRequest,
        UserInfo,
        PairInfo,
        TokenGenRequest,
        AuthCheckResponse,
        AssignmentInfo,
        LockerResisterRequest,
        LoginFormRequest,
        LockerStatusResponse,
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
#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckRequest {
    #[schema(example = "Hello world from json!")]
    pub text: String,
}

#[utoipa::path(context_path = "/api")]
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
#[utoipa::path(context_path = "/api/locker")]
#[post("/token-gen", data = "<request>")]
pub async fn token_generator(request: Json<TokenGenRequest>, app: &State<App>) -> Status {

    let data = &request.data;

    let token = match app.auth.register(&data.main_user.clone(), &data.co_user.clone(), false).await{
        Ok(auth) => auth.main_auth_token,
        Err(_) => return Status::InternalServerError,
    };

    // メール内容の作成
    let main_user = &data.main_user;

    dotenv().ok();
    let app_url = env::var("APP_URL").expect("APP_URL must be set.");

    let user_address = format!("{}@ed.tus.ac.jp", main_user.student_id);
    let content = format!("{}{} 様\n\n以下のURLにアクセスして認証を完了してください。\n{}/locker/user-register?method=1&token={}", main_user.family_name, main_user.given_name, app_url, token);
    let subject = "ロッカーシステム メール認証";

    if app.auth.mail_sender(user_address, content, subject).await.is_err(){
        return Status::InternalServerError;
    }

    Status::Created
}

// main_user認証API {
#[utoipa::path(context_path = "/api/locker")]
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

    // メール内容の作成
    dotenv().ok();
    let app_url = env::var("APP_URL").expect("APP_URL must be set.");

    let user_address = format!("{}@ed.tus.ac.jp", co_user.student_id);
    let content = format!("{}{} 様\n\n以下のURLにアクセスして認証を完了してください。\n{}/locker/user-register?method=0&token={}", co_user.family_name, co_user.given_name, app_url, auth.co_auth_token);
    let subject = "ロッカーシステム メール認証";

    if app.auth.mail_sender(user_address, content, subject).await.is_err(){
        return Status::InternalServerError;
    }

    Status::Created
}

// co_user認証API {
#[utoipa::path(context_path = "/api/locker")]
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

    let token = match app.auth.register(&main_user.clone(), &co_user.clone(), true).await{
        Ok(auth) => auth.main_auth_token,
        Err(_) => return Status::InternalServerError,
    };

    dotenv().ok();
    let app_url = env::var("APP_URL").expect("APP_URL must be set.");

    let user_address = format!("{}@ed.tus.ac.jp", main_user.student_id);
    let content = format!("認証が完了しました。\n以下のリンクから使用するロッカー番号を選択してください。\n\n{}/locker/user-register/?method=2&token={}", app_url, token);
    let subject = "ロッカーシステム 認証完了通知";

    if app.auth.mail_sender(user_address, content, subject).await.is_err(){
        return Status::InternalServerError;
    }

    Status::Created
}

#[derive(Serialize, ToSchema)]
pub struct AuthCheckResponse {
    pub data: PairInfo,
}

// 認証検証API
#[utoipa::path(context_path = "/api/locker")]
#[get("/auth-check?<token>")]
pub async fn auth_check(token: String, app: &State<App>) -> Result<Json<AuthCheckResponse>, Status> {
    let auth = match app.auth.token_check(token, true).await{
        Ok(auth) => auth,
        Err(status) => return Err(status),
    };

    let main_user = &UserInfo{
        student_id: auth.main_student_id.clone(),
        family_name: auth.main_family_name.clone(),
        given_name: auth.main_given_name.clone(),
    };

    let co_user = &UserInfo{
        student_id: auth.co_student_id.clone(),
        family_name: auth.co_family_name.clone(),
        given_name: auth.co_given_name.clone(),
    };

    let student_pair = &PairInfo{
        main_user: main_user.clone(),
        co_user: co_user.clone(),
    };

    Ok(Json(AuthCheckResponse{
        data: student_pair.clone(),
    }))

}

/// ### ロッカー状態
///
/// ロッカー空き状態確認APIのレスポンスデータに使用
#[derive(Clone, Serialize, ToSchema)]
pub struct LockerStatus{
    pub locker_id: String,
    pub floor: i8,
    pub status: String,
}

/// ### ロッカー空き状態確認APIのレスポンスデータ
#[derive(Serialize, ToSchema)]
pub struct LockerStatusResponse{
    pub data: Vec<LockerStatus>,
}

/// ### ロッカー空き状態確認API
#[utoipa::path(context_path = "/api/locker")]
#[post("/availability?<floor>")]
pub async fn availability(floor: Option<i8>, app: &State<App>) -> Result<Json<LockerStatusResponse>, Status> {
    // 指定階数のlockerレコードの取得
    let result = app.locker.get_by_floor(&floor).await.unwrap();

    let mut response: Vec<LockerStatus> = Vec::new();
    for element in result {
        let data = LockerStatus{
            locker_id: element.locker_id.clone(),
            floor: element.locker_id.chars().nth(0).unwrap().to_digit(10).unwrap() as i8,
            status: element.status,
        };
        response.push(data);
    }

    Ok(Json(LockerStatusResponse{
        data: response,
    }))
}

#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LockerResisterRequest {
    pub data: AssignmentInfo,
}

/// ### ロッカー登録API
#[utoipa::path(context_path = "/api/locker")]
#[post("/locker-register", data = "<request>")]
pub async fn locker_register(request: Json<LockerResisterRequest>, app: &State<App>) -> (Status, &'static str) {

    let assignment = &request.data;

    // pair_idの検索
    let user_pair = match app.student_pair.get_by_main_id(&assignment.student_id).await {
        Ok(student_pair) => student_pair,
        Err(_) => return (Status::InternalServerError, "failed to get student_pair id"),
    };

    // 対象ロッカーの空き確認
    let locker = match app.locker.get_by_id(&assignment.locker_id).await {
        Ok(locker) => locker,
        Err(_) => return (Status::InternalServerError, "failed to get locker"),
    };

    if locker.status != "vacant" {
        return (Status::BadRequest, "This locker is not vacant");
    }

    // 割り当て情報の登録
    if app.assignment_record.register(&user_pair, assignment).await.is_err() {
        return (Status::InternalServerError, "failed to insert request");
    }

    // ロッカーのステータス更新
    let status = String::from("occupied");
    if app.locker.update_status(&assignment.locker_id, &status).await.is_err() {
        return (Status::InternalServerError, "failed to update locker status");
    }

    (Status::Created, "success create assignment")
}

/// ### 管理者パスワード照合APIのリクエストデータ
///
/// username    : ユーザ名
///
/// password    : パスワード
#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginFormRequest{
    #[schema(example = "user000")]
    pub username : String,
    #[schema(example = "0000")]
    pub password : String,
}

/// ### JWTペイロードに指定する構造体
///
/// subject     : tokenの持ち主
///
/// expire      : tokenの持続時間
///
/// issued at   : tokenの発行時刻
#[derive(Serialize, Deserialize)]
pub struct Claims{
    sub: String,
    exp: usize,
    iat: usize,
}

/// ### 管理者パスワード照合API
#[utoipa::path(context_path = "/api")]
#[post("/login", data = "<request>")]
pub async fn login(request: Json<LoginFormRequest>, jar: &CookieJar<'_>, app: &State<App>) -> Status {

    // usernameが一致するレコードをadminテーブルから取得
    let credential = match app.admin.get_by_name(&request.username).await {
        Ok(admin) => admin,
        Err(_) => return Status::InternalServerError,
    };

    // 環境変数TOKEN_KEYを取得
    dotenv().ok();
    let key = env::var("TOKEN_KEY").expect("token key must be set.");

    // passwordの検証
    if request.password != credential.password {
        return Status::InternalServerError
    }

    // jwtの発行

    // headerの宣言
    let mut header = Header::default();

    // 使用するトークンはjwt
    header.typ = Some("JWT".to_string());

    // 使用するアルゴリズムはHMAC SHA-256
    header.alg = Algorithm::HS256;

    // 現在時刻を取得
    let now = Utc::now();

    // claimsを設定
    let admin_claims = Claims{
        sub: request.username.clone(),
        exp: (now + Duration::hours(1)).timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    // jwtを発行
    let token = encode(&header, &admin_claims, &EncodingKey::from_secret(key.as_ref())).unwrap();

    // cookieを作成
    let cookie = Cookie::build(("token", token))
        .path("/")
        .secure(false)
        .http_only(true);

    jar.add(cookie);

    return Status::Created
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct UserSearchResult {
    pub locker_id : String,
    pub floor : i8,
    pub main_user : UserInfo,
    pub co_user : UserInfo,
    pub year : i32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct UserSearchResponce {
    pub data: Vec<UserSearchResult>,
}

/// ロッカー利用者検索API
///
/// nameは申請者の名前のみ受け付ける
#[utoipa::path(context_path = "/api/admin")]
#[get("/user-search/<year>?<floor>&<familyname>&<givenname>")]
pub async fn user_search(year: i32, floor: Option<i8>, familyname: Option<String>, givenname: Option<String>, jar: &CookieJar<'_>, app: &State<App>) -> Result<Json<UserSearchResponce>, Status> {

    // Cookieからjwtの取得
    let jwt = match jar.get("token").map(|c| c.value()) {
        None => return Err(Status::BadRequest),
        Some(t) => String::from(t),
    };

    // jwtの検証
    match decode_jwt(&jwt) {
        None => return Err(Status::BadRequest),
        Some(_) => {
            let family_name_val = match familyname {
                None => String::from(""),
                Some(x) => String::from(RawStr::new(&x).url_decode().unwrap()),
            };
            let given_name_val = match givenname {
                None => String::from(""),
                Some(x) => String::from(RawStr::new(&x).url_decode().unwrap()),
            };

            let match_user = match app.student.get_by_name(&family_name_val, &given_name_val).await {
                Ok(student) => student,
                Err(_) => return Err(Status::NotFound),
            };

            let mut user_pairs= Vec::new();
            for element in match_user {
                let user_pair = match app.student_pair.get_by_id(&element.student_id).await {
                    Ok(student_pair) => student_pair,
                    Err(_) => return Err(Status::NotFound),
                };
                user_pairs.push(user_pair)
            }

            let unique_user_pair: HashSet<StudentPair> = user_pairs.into_iter().collect();

            let mut matched_record: Vec<AssignmentRecord> = Vec::new();
            for element in unique_user_pair {
                let mut get_result = match app.assignment_record.get(&year, floor, &element.pair_id).await {
                    Ok(res) => res,
                    Err(_) => return Err(Status::NotFound),
                };
                matched_record.append(&mut get_result);
            }

            let mut result: Vec<UserSearchResult> = Vec::new();

            for element in matched_record {
                let pair = match app.student_pair.get_by_pair_id(&element.pair_id).await {
                    Ok(studentpair) => studentpair,
                    Err(_) => return Err(Status::NotFound),
                };

                let main_user = match app.student.get_by_id(&pair.student_id1).await {
                    Ok(student) => student,
                    Err(_) => return Err(Status::NotFound),
                };

                let co_user = match app.student.get_by_id(&pair.student_id2).await {
                    Ok(student) => student,
                    Err(_) => return Err(Status::NotFound),
                };

                let main_user_info = UserInfo {
                    student_id: main_user.student_id.clone(),
                    family_name: main_user.family_name.clone(),
                    given_name: main_user.given_name.clone(),
                };

                let co_user_info = UserInfo {
                    student_id: co_user.student_id,
                    family_name: co_user.family_name,
                    given_name: co_user.given_name,
                };

                let locker_id_borrow = element.locker_id.clone();

                let num = UserSearchResult {
                    locker_id: element.locker_id,
                    floor: locker_id_borrow.chars().nth(0).unwrap().to_digit(10).unwrap() as i8,
                    main_user: main_user_info,
                    co_user: co_user_info,
                    year: year,
                };

                result.push(num);
            }

            Ok(Json(UserSearchResponce{
                data: result,
            }))
        }
    }
}
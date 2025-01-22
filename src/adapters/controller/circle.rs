use crate::adapters::httpmodels::*;
use crate::domain::{circle::{OrganizationInfo, Organization}, student::RepresentativeInfo, googleapis::{MakeContact, MakeContactParameters}};
use crate::infrastructure::router::App;
use crate::usecase::time::TimeUsecase;
use crate::usecase::{
                    auth::AuthUsecase,
                    representatives::RepresentativesUsecase,
                    organization::OrganizationUsecase,
                    registration::RegistrationUsecase,
                    };
use crate::utils::decode_jwt::decode_jwt;
use crate::utils::oauth_authentication::refresh_access_token;

use std::env;
use std::time::Duration;
use chrono::{DateTime, Utc};
use dotenv::dotenv;
use rocket::{get, http::{Status, CookieJar}, post, serde::json::Json, State};
use regex::Regex;
use reqwest::StatusCode;

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
#[utoipa::path(context_path = "/api/circle")]
#[post("/co-auth?<token>&<id>")]
pub async fn circle_co_auth(token: String, id: Option<String>, app:&State<App>) -> (Status, &'static str) {

    // tokenが一致するレコードを取得
    let auth = match app.auth.token_check(token, false).await{
        Ok(auth) => auth,
        // 存在しなかったら終了
        Err(status) => return (status, "invalid token"),
    };

    // authのphaseを確認
    if auth.phase != String::from("co_auth") {
        return (Status::BadRequest, "authentication phase does not match");
    }

    // auth_infoからレコードを取得
    let auth_info = match app.auth.get_circle_auth_info(&auth.auth_id).await {
        Ok(info) => info,
        Err(status) => return (status, "failed to get circle info"),
    };

    // main_userの情報を格納
    let main_user= RepresentativeInfo{
        student_id: auth_info.main_student_id,
        family_name: auth_info.main_family_name,
        given_name: auth_info.main_given_name,
        email: auth_info.main_email,
        phone_number: auth_info.main_phone
    };

    // co_userの情報を格納
    let co_user = RepresentativeInfo{
        student_id: auth_info.co_student_id,
        family_name: auth_info.co_family_name,
        given_name: auth_info.co_given_name,
        email: auth_info.co_email,
        phone_number: auth_info.co_phone,
    };

    // organizationの情報を格納
    let organization = Organization{
        organization_name: auth_info.organization_name,
        organization_ruby: auth_info.organization_ruby,
        organization_email: auth_info.organization_email,
    };

    if app.representatives.register(&co_user).await.is_err() {
        return (Status::InternalServerError, "failed to insert Representative")
    }

    match id.clone() {
        // 団体情報更新
        Some(id) => {
            // 更新処理

            // organization_idの整形
            let re = Regex::new(r"[1-9]+").unwrap();
            let organization_id = match re.find(id.as_str()) {
                Some(m) => m.as_str().parse::<i32>().unwrap(),
                None => {return (Status::InternalServerError, "can't get valid organization_id")}
            };

            // 団体メールアドレスの更新
            if organization.organization_email != "" {
                if app.organization.update_email(&organization_id, &organization.organization_email).await.is_err() {
                    return (Status::InternalServerError, "failed to update organization")
                }
            }

            // 代表者、副代表者の更新
            if main_user.student_id != "" && co_user.student_id != "" {
                if app.registration.update_student(&organization_id, &main_user.student_id, &co_user.student_id).await.is_err() {
                    return (Status::InternalServerError, "failed to update registration")
                }
            }
        },
        // 団体新規登録
        None => {

            // registrationの情報を格納
            let organization_info = &OrganizationInfo{
                organization: organization.clone(),
                main_user: main_user.clone(),
                co_user: co_user.clone(),
                b_doc: auth_info.b_doc.clone(),
                c_doc: auth_info.c_doc.clone(),
                d_doc: auth_info.d_doc.clone(),
            };

            let organization_id = match app.organization.register(&organization).await {
                Ok(org) => {
                    org.organization_id
                },
                Err(_) => {
                    return (Status::InternalServerError, "failed to insert Organization")
                }
            };

            // register DBに登録
            if app.registration.register(organization_info, &organization_id).await.is_err() {
                return (Status::InternalServerError, "failed to insert Registration")
            }
        }
    }

    // googleapiの処理

    // token類を環境変数から取得
    dotenv().ok();
    let refresh_token = env::var("REFRESH_TOKEN").expect("refresh token must be set.");
    let client_id = env::var("CLIENT_ID").expect("client id must be set.");
    let client_secret = env::var("CLIENT_SECRET").expect("client secret must be set.");
    let deploy_id = env::var("DEPLOY_ID").expect("deploy id must be set.");

    // refreshtokenからaccesstokenを取得
    let api_tokens = refresh_access_token(refresh_token.as_str(), client_id.as_str(), client_secret.as_str()).await.unwrap();

    // googleapi用のデータを格納
    let input = MakeContact {
        function: String::from("onPostRequest"),
        parameters: MakeContactParameters {
            data: OrganizationInfo{
                organization: organization,
                main_user: main_user.clone(),
                co_user: co_user,
                b_doc: auth_info.b_doc,
                c_doc: auth_info.c_doc,
                d_doc: auth_info.d_doc,
            },
        }
    };
    let input_json = serde_json::to_string(&input).unwrap();

    // リクエスト用クライアント
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(60))
        .build().unwrap();

    // リクエストを送信
    let result = client.post(format!("https://script.googleapis.com/v1/scripts/{}:run", deploy_id.as_str()))
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", api_tokens.access_token))
        .body(input_json)
        .send()
        .await.unwrap();

    match result.status() {
        StatusCode::OK => {
            let user_address = main_user.email.to_string();
            let content = match id {
                Some(_) => format!("{}{} 様\n\nメール認証が完了し、団体情報が更新されました。\n", main_user.family_name, main_user.given_name),
                None => format!("{}{} 様\n\nメール認証が完了し、団体情報が登録されました。\n", main_user.family_name, main_user.given_name),
            };
            let subject = "団体登録システム メール認証完了のお知らせ";

            // 登録完了メールの送信
            if app.auth.mail_sender(user_address, content, subject).await.is_err() {
                return (Status::InternalServerError, "failed to send authentication email");
            }

            (Status::Created, "Organization Infomation updated successfully")
        },
        _ => {
            (Status::InternalServerError, "googleapi request failed")
        }
    }
}

// 団体情報取得API
#[utoipa::path(context_path = "/api/circle")]
#[get("/status")]
pub async fn circle_status(app: &State<App>) -> Result<Json<OrganizationStatusResponse>, Status> {

    let result = app.registration.get_all().await.unwrap();

    let mut response: Vec<OrganizationStatus> = Vec::new();
    for element in result {
        let organization_info = app.organization.get_by_id(&element.organization_id).await.unwrap();
        let data = OrganizationStatus{
            organization_id: format!("C{0: >05}", element.organization_id),
            organization_name: organization_info.organization_name,
            status_acceptance: element.status_acceptance,
            status_authentication: element.status_authentication,
            status_form_confirmation: element.status_form_confirmation,
            status_registration_complete: element.status_registration_complete,
        };
        response.push(data);
    }

    response.sort_by(|lt, rt| lt.organization_id.partial_cmp(&rt.organization_id).unwrap());

    Ok(Json(OrganizationStatusResponse {
        data: response,
    }))
}

// 団体アクセス制限API POST
#[utoipa::path(context_path = "/api/admin/circle")]
#[post("/access/setting", data="<request>")]
pub async fn access_setting_post(request: Json<CircleAccessSetting>, jar: &CookieJar<'_>, app: &State<App>) -> (Status, &'static str) {
    // Cookieからjwtの取得
    let jwt = match jar.get("token").map(|c| c.value()) {
        None => return (Status::Unauthorized, "request is unautorized"),
        Some(t) => String::from(t),
    };

    match decode_jwt(&jwt) {
        None => return (Status::Unauthorized, "request token is not valid."),
        Some(_) => {
            // 時間情報を整形
            let start_time = DateTime::parse_from_rfc3339(&request.start).unwrap().naive_utc();
            let end_time = DateTime::parse_from_rfc3339(&request.end).unwrap().naive_utc();

            // アクセス制限情報をDBに保存
            if app.time.register(&String::from("access_restrictions"), &start_time, &end_time).await.is_err() {
                return (Status::InternalServerError, "failed to insert time")
            }

            (Status::Created, "Access Restrictions registered successfully")
        }
    }
}

// 団体アクセス制限API GET
#[utoipa::path(context_path = "/api/circle")]
#[get("/access/setting")]
pub async fn access_setting_get(app: &State<App>) -> Result<Json<CircleAccessSetting>, Status> {

    // nameがaccess_restrictionsのレコードをtimeから取得しレスポンスを作成
    let response =  match app.time.get_by_name(&String::from("access_restrictions")).await {
        Ok(time) => {
            CircleAccessSetting {
                start: time.start_time.and_local_timezone(Utc).unwrap().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                end: time.end_time.and_local_timezone(Utc).unwrap().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
            }
        },
        Err(_) => {
            CircleAccessSetting {
                start: String::from(""),
                end: String::from(""),
            }
        }
    };

    Ok(Json(response))
}

// 管理者用団体情報取得API
#[utoipa::path(context_path = "/api/admin/circle")]
#[get("/list")]
pub async fn circle_list(app: &State<App>) -> Result<Json<OrganizationListResponse>, Status> {
    let result = app.registration.get_all().await.unwrap();

    let mut response: Vec<OrganizationList> = Vec::new();
    for element in result {
        let organization_info = app.organization.get_by_id(&element.organization_id).await.unwrap();
        let main_info = app.representatives.get_by_id(&element.main_student_id).await.unwrap();
        let co_info = app.representatives.get_by_id(&element.co_student_id).await.unwrap();

        let data = OrganizationList{
            organization_id: format!("C{0: >05}", element.organization_id),
            organization_name: organization_info.organization_name,
            organization_email: organization_info.organization_email,
            main_id: main_info.student_id,
            main_family_name: main_info.family_name,
            main_given_name: main_info.given_name,
            main_email: main_info.email,
            main_phone: main_info.phone,
            co_id: co_info.student_id,
            co_family_name: co_info.family_name,
            co_given_name: co_info.given_name,
            co_email: co_info.email,
            co_phone: co_info.phone,
            b_url: element.b_doc,
            c_url: element.c_doc,
            d_url: element.d_doc,
            status_acceptance: element.status_acceptance,
            status_authentication: element.status_authentication,
            status_form_confirmation: element.status_form_confirmation,
            status_registration_complete: element.status_registration_complete,
        };
        response.push(data);
    }

    response.sort_by(|lt, rt| lt.organization_id.partial_cmp(&rt.organization_id).unwrap());

    Ok(Json(OrganizationListResponse {
        data: response,
    }))
}
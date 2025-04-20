use crate::adapters::httpmodels::*;
use crate::domain::{student::UserInfo, student_pair::PairInfo};
use crate::infrastructure::{router::App, models::{AssignmentRecord, StudentPair}};
use crate::usecase::{
                    student::StudentUsecase,
                    student_pair::StudentPairUsecase,
                    assignment_record::AssignmentRecordUsecase,
                    auth::AuthUsecase,
                    locker::LockerUsecase,
                    admin::AdminUsecase};
use crate::utils::{jwt::{encode_jwt, decode_jwt}, verify_password::verify_password_hash, verify_recaptcha::verify_recaptcha};

use std::{env, collections::HashSet};
use uuid::Uuid;
use dotenv::dotenv;
use rocket::{get, http::{Status, RawStr, Cookie, CookieJar, SameSite}, post, serde::json::Json, State};
use rocket::time::Duration as RocketDuration;
use chrono::Duration as ChronoDuration;
use regex::Regex;

// token生成、メール送信API
#[utoipa::path(context_path = "/api/locker")]
#[post("/token-gen", data = "<request>")]
pub async fn token_generator(request: Json<LockerTokenGenRequest>, app: &State<App>) -> Status {

    let data = &request.data;

    // データのバリデーション

    // 学籍番号についてのバリデーション
    let re = Regex::new(r"^(15\d{5}|[48][1-6]\d{5})$").unwrap();
    if !(re.is_match(data.main_user.student_id.clone().as_str())) {
        return Status::BadRequest;
    }
    if !(re.is_match(data.co_user.student_id.clone().as_str())) {
        return Status::BadRequest;
    }
    if !app.option.same_student_enable && data.main_user.student_id.clone() == data.co_user.student_id.clone() {
        return Status::BadRequest;
    }

    // 氏名についてのバリデーション
    let re = Regex::new(r"^[A-Za-z\p{Kana}\p{Hira}\p{Han}]+$").unwrap();
    if !(re.is_match(data.main_user.family_name.clone().as_str())) {
        return Status::BadRequest;
    }
    if !(re.is_match(data.main_user.given_name.clone().as_str())) {
        return Status::BadRequest;
    }
    if !(re.is_match(data.co_user.family_name.clone().as_str())) {
        return Status::BadRequest;
    }
    if !(re.is_match(data.co_user.given_name.clone().as_str())) {
        return Status::BadRequest;
    }

    dotenv().ok();

    // recaptchaの検証
    if !verify_recaptcha(&request.recaptcha_token).await.unwrap_or(false) {
        return Status::Unauthorized;
    }


    // tokenの生成
    let token = match app.auth.locker_register(&data.main_user.clone(), &data.co_user.clone(), &String::from("main_auth"), false).await{
        Ok(auth) => auth.main_auth_token,
        Err(_) => return Status::InternalServerError,
    };

    // メール内容の作成
    let main_user = &data.main_user;

    let app_url = env::var("APP_URL").expect("APP_URL must be set.");

    let user_address = format!("{}@ed.tus.ac.jp", main_user.student_id);
    let content = format!("{}{} 様\n\n以下のURLにアクセスして認証を完了してください。\n\n{}/locker/user-register?method=1&token={}", main_user.family_name, main_user.given_name, app_url, token);
    let subject = "ロッカーシステム メール認証";

    if app.option.local_mail_enable {
        if app.auth.mail_sender_local(user_address, content, subject).await.is_err(){
            return Status::InternalServerError;
        }
    }
    else {
        if app.auth.mail_sender(user_address, content, subject).await.is_err(){
            return Status::InternalServerError;
        }
    }

    Status::Created
}

// main_user認証API
#[utoipa::path(context_path = "/api/locker")]
#[get("/main-auth?<token>")]
pub async fn main_auth(token: String, app: &State<App>) -> Status {
    //データのバリデーション

    // token

    let re = Regex::new(r"^[a-zA-Z0-9]{16}$").unwrap();
    if !(re.is_match(token.as_str())) {
        return Status::BadRequest;
    }

    // tokenが一致するレコードを取得
    let auth = match app.auth.token_check(token, true).await{
        Ok(auth) => auth,
        // 存在しなかったら終了
        Err(status) => return status,
    };

    // authのphaseを確認
    if auth.phase != *"main_auth" {
        return Status::BadRequest
    }

    let auth_info = match app.auth.get_locker_auth_info(&auth.auth_id).await {
        Ok(info) => info,
        Err(status) => return status,
    };

    // mainuserの情報を格納
    let main_user = &UserInfo{
        student_id: auth_info.main_student_id.clone(),
        family_name: auth_info.main_family_name.clone(),
        given_name: auth_info.main_given_name.clone(),
    };

    // mainuserの情報をstudentテーブルに保存
    if app.student.register(&main_user.clone()).await.is_err(){
        return Status::InternalServerError;
    }

    // couserの情報を格納
    let co_user = &UserInfo{
        student_id: auth_info.co_student_id.clone(),
        family_name: auth_info.co_family_name.clone(),
        given_name: auth_info.co_given_name.clone(),
    };

    // メール内容の作成
    dotenv().ok();
    let app_url = env::var("APP_URL").expect("APP_URL must be set.");

    let user_address = format!("{}@ed.tus.ac.jp", co_user.student_id);
    let content = format!("{}{} 様\n\n以下のURLにアクセスして認証を完了してください。\n\n{}/locker/user-register?method=0&token={}", co_user.family_name, co_user.given_name, app_url, auth.co_auth_token);
    let subject = "ロッカーシステム メール認証";

    // メールの送信
    if app.option.local_mail_enable {
        if app.auth.mail_sender_local(user_address, content, subject).await.is_err(){
            return Status::InternalServerError;
        }
    }
    else {
        if app.auth.mail_sender(user_address, content, subject).await.is_err(){
            return Status::InternalServerError;
        }
    }

    // phaseの更新
    if app.auth.update_phase(&auth.auth_id, String::from("co_auth")).await.is_err() {
        return Status::InternalServerError;
    }

    Status::Created
}

// co_user認証API {
#[utoipa::path(context_path = "/api/locker")]
#[get("/co-auth?<token>")]
pub async fn co_auth(token: String, app: &State<App>) -> Status {
    //データのバリデーション

    // token

    let re = Regex::new(r"^[a-zA-Z0-9]{16}$").unwrap();
    if !(re.is_match(token.as_str())) {
        return Status::BadRequest;
    }

    // tokenが一致するレコードを取得
    let auth = match app.auth.token_check(token, false).await{
        Ok(auth) => auth,
        Err(status) => return status,
    };

    // authのphaseを確認
    if auth.phase != *"co_auth" {
        return Status::BadRequest
    }

    let auth_info = match app.auth.get_locker_auth_info(&auth.auth_id).await {
        Ok(info) => info,
        Err(status) => return status,
    };

    // couserの情報を格納
    let co_user = &UserInfo{
        student_id: auth_info.co_student_id.clone(),
        family_name: auth_info.co_family_name.clone(),
        given_name: auth_info.co_given_name.clone(),
    };

    // couserの情報をstudentテーブルに保存
    if app.student.register(&co_user.clone()).await.is_err(){
        return Status::InternalServerError;
    }

    // mainuserの情報を格納
    let main_user = &UserInfo{
        student_id: auth_info.main_student_id.clone(),
        family_name: auth_info.main_family_name.clone(),
        given_name: auth_info.main_given_name.clone(),
    };

    // studentpairの情報を作成
    let student_pair = &PairInfo{
        main_user: main_user.clone(),
        co_user: co_user.clone(),
    };

    // studentpairに同じ学籍番号がないか確認
    match app.student_pair.get_by_id(&main_user.student_id).await {
        Ok(None) => {},
        Ok(_) => return Status::InternalServerError,
        Err(status) => return status,
    }

    match app.student_pair.get_by_id(&co_user.student_id).await {
        Ok(None) => {},
        Ok(_) => return Status::InternalServerError,
        Err(status) => return status,
    }

    // studentpairの情報をstudent_pairテーブルに保存
    if app.student_pair.register(student_pair).await.is_err(){
        return Status::InternalServerError;
    }

    // 認証完了用のレコードを保存
    let token = match app.auth.locker_register(&main_user.clone(), &co_user.clone(), &String::from("auth_check"), true).await{
        Ok(auth) => auth.main_auth_token,
        Err(_) => return Status::InternalServerError,
    };

    // メールの作成
    dotenv().ok();
    let app_url = env::var("APP_URL").expect("APP_URL must be set.");

    let user_address = format!("{}@ed.tus.ac.jp", main_user.student_id);
    let content = format!("認証が完了しました。\n以下のリンクから使用するロッカー番号を選択してください。\n\n{}/locker/user-register/?method=2&token={}", app_url, token);
    let subject = "ロッカーシステム 認証完了通知";

    // メールの送信
    if app.option.local_mail_enable {
        if app.auth.mail_sender_local(user_address, content, subject).await.is_err(){
            return Status::InternalServerError;
        }
    }
    else {
        if app.auth.mail_sender(user_address, content, subject).await.is_err(){
            return Status::InternalServerError;
        }
    }

    // レコードを削除
    if app.auth.delete(&auth.auth_id).await.is_err() {
        return Status::InternalServerError;
    }

    Status::Created
}

// 認証検証API
#[utoipa::path(context_path = "/api/locker")]
#[get("/auth-check?<token>")]
pub async fn auth_check(token: String, app: &State<App>) -> Result<Json<AuthCheckResponse>, Status> {
    //データのバリデーション

    // token

    let re = Regex::new(r"^[a-zA-Z0-9]{16}$").unwrap();
    if !(re.is_match(token.as_str())) {
        return Err(Status::BadRequest);
    }

    // tokenを取得
    let auth = match app.auth.token_check(token, true).await{
        Ok(auth) => auth,
        Err(status) => return Err(status),
    };

    // authのphaseを確認
    if auth.phase != *"auth_check" {
        return Err(Status::BadRequest)
    }

    let auth_info = match app.auth.get_locker_auth_info(&auth.auth_id).await {
        Ok(info) => info,
        Err(status) => return Err(status),
    };

    // mainuserの情報を格納
    let main_user = &UserInfo{
        student_id: auth_info.main_student_id.clone(),
        family_name: auth_info.main_family_name.clone(),
        given_name: auth_info.main_given_name.clone(),
    };

    // couserの情報を格納
    let co_user = &UserInfo{
        student_id: auth_info.co_student_id.clone(),
        family_name: auth_info.co_family_name.clone(),
        given_name: auth_info.co_given_name.clone(),
    };

    // studentpairの情報を作成
    let student_pair = &PairInfo{
        main_user: main_user.clone(),
        co_user: co_user.clone(),
    };

    Ok(Json(AuthCheckResponse{
        data: student_pair.clone(),
        auth_id: auth.auth_id.clone().to_string(),
    }))
}

/// ### ロッカー空き状態確認API
#[utoipa::path(context_path = "/api/locker")]
#[get("/availability?<floor>")]
pub async fn availability(floor: Option<i8>, app: &State<App>) -> Result<Json<LockerStatusResponse>, Status> {
    // データのバリデーション

    // floor
    if let Some(floor) = floor {
        let re = Regex::new(r"^[2-6]$").unwrap();
        if !(re.is_match(&floor.to_string())) {
            return Err(Status::BadRequest);
        }
    }

    // 指定階数のlockerレコードの取得
    let result = app.locker.get_by_floor(&floor).await.unwrap();

    let mut response: Vec<LockerStatus> = Vec::new();
    for element in result {
        let data = LockerStatus{
            locker_id: element.locker_id.clone(),
            floor: element.locker_id.chars().next().unwrap().to_digit(10).unwrap() as i8,
            status: element.status,
        };
        response.push(data);
    }

    response.sort_by(|lt, rt| lt.locker_id.partial_cmp(&rt.locker_id).unwrap());

    Ok(Json(LockerStatusResponse{
        data: response,
    }))
}

/// ### ロッカー登録API
#[utoipa::path(context_path = "/api/locker")]
#[post("/locker-register", data = "<request>")]
pub async fn locker_register(request: Json<LockerResisterRequest>, app: &State<App>) -> (Status, &'static str) {

    let assignment = &request.data;

    //データのバリデーション

    // 代表者学籍番号
    let re = Regex::new(r"^(15\d{5}|[48][1-6]\d{5})$").unwrap();
    if !(re.is_match(assignment.student_id.as_str())) {
        return (Status::BadRequest, "request data is not valid");
    }

    // ロッカー番号
    let re = Regex::new(r"^[2-6]\d{3}$").unwrap();
    if !(re.is_match(assignment.locker_id.as_str())) {
        return (Status::BadRequest, "request data is not valid");
    }

    // authID
    let auth_id = match Uuid::parse_str(&request.auth_id) {
        Ok(uuid) => {uuid},
        Err(_) => {return (Status::BadRequest, "request auth_id is not valid");}
    };

    // pair_idの検索
    let user_pair = match app.student_pair.get_by_main_id(&assignment.student_id).await {
        Ok(student_pair) => student_pair,
        Err(_) => return (Status::InternalServerError, "failed to get student_pair id"),
    };

    // 既に登録されていないかの確認
    match app.assignment_record.get_by_pair_id(&user_pair.pair_id).await {
        Ok(Some(_)) => {return (Status::InternalServerError, "same pair already exists")},
        Ok(None) => {},
        Err(_) => {return (Status::InternalServerError, "failed to get assignment_record")},
    }

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

    // レコードを削除
    if app.auth.delete(&auth_id).await.is_err() {
        return (Status::InternalServerError, "failed to delete auth table");
    }

    let user_address = format!("{}@ed.tus.ac.jp", user_pair.student_id1.clone());
    let content = format!(
        "ロッカーの登録が完了しました。\n\n\
        ロッカー番号: {}\n\n\
        【内容物の回収・保管・廃棄について】\n\
        ・内容物回収期間：3月中旬\n\
        ・保管期間：次年度の4~6月\n\
        ・廃棄日：次年度の6月下旬\n\n\
        ※ 期限までに回収しなかった場合、内容物は廃棄され、返還できません。\n\
        ※ 廃棄に伴う責任は負いかねますので、必ず期間内に回収をお願いします。\n\n\
        【ロッカー使用時の注意事項】\n\
        ・ロッカー使用時には必ず鍵を使用してください。\n\
        ・鍵の購入はこちら： https://www.univcoop.jp/rikadai/time/index.html#s02 \n\n\
        ご不明点がございましたら、お問い合わせください。\n\n\
        よろしくお願いいたします。\n",
        assignment.locker_id
    );
    let subject = "ロッカーシステム ロッカー登録通知";

    // メールの送信
    if app.option.local_mail_enable {
        if app.auth.mail_sender_local(user_address, content, subject).await.is_err(){
            return (Status::InternalServerError, "Failed to send authentication email");
        }
    }
    else {
        if app.auth.mail_sender(user_address, content, subject).await.is_err(){
            return (Status::InternalServerError, "Failed to send authentication email");
        }
    }

    println!(r"( 'ω')/ウオオオオオアアアーーーーッ！！！");

    (Status::Created, "success create assignment")
}

/// ### 管理者パスワード照合API
#[utoipa::path(context_path = "/api")]
#[post("/login", data = "<request>")]
pub async fn login(request: Json<LoginFormRequest>, jar: &CookieJar<'_>, app: &State<App>) -> Status {
    // バリデーション

    // username
    let re = Regex::new(r"^[A-Za-z\d_-]+$").unwrap();
    if !(re.is_match(request.username.as_str())) {
        return Status::BadRequest;
    }

    // password
    let re = Regex::new(r"^[A-Za-z\d]+$").unwrap();
    if !(re.is_match(request.password.as_str())) {
        return Status::BadRequest;
    }

    // usernameが一致するレコードをadminテーブルから取得
    let credential = match app.admin.get_by_name(&request.username).await {
        Ok(admin) => admin,
        Err(_) => return Status::InternalServerError,
    };

    // passwordの検証
    match verify_password_hash(request.password.clone(), credential.password) {
        Ok(_) => {},
        Err(_) => {
            return Status::BadRequest},
    }

    // 環境変数TOKEN_KEYを取得
    dotenv().ok();
    let key = env::var("TOKEN_KEY").expect("token key must be set.");
    let domain = env::var("DOMAIN").expect("domain must be set.");

    let token = encode_jwt(&request.username, ChronoDuration::hours(1), &key);

    // cookieを作成
    let cookie = Cookie::build(("token", token))
        .path("/")
        .domain(domain)
        .max_age(RocketDuration::hours(1))
        .secure(true)
        .same_site(SameSite::Strict)
        .http_only(true);

    jar.add(cookie);

    Status::Created
}

/// ### 管理者ログアウトAPI
#[utoipa::path(context_path = "/api")]
#[post("/logout")]
pub async fn logout(jar: &CookieJar<'_>) -> Status {

    dotenv().ok();
    let domain = env::var("DOMAIN").expect("domain must be set.");

    let expired_cookie = Cookie::build(("token", ""))
        .path("/")
        .domain(domain)
        .max_age(RocketDuration::seconds(0)) // 即無効化する
        .secure(true)
        .same_site(SameSite::Strict)
        .http_only(true);

    jar.add(expired_cookie);

    Status::Ok
}

/// ロッカー利用者検索API
///
/// nameは申請者の名前のみ受け付ける
#[utoipa::path(context_path = "/api/admin/locker")]
#[get("/user-search/<year>?<floor>&<familyname>&<givenname>")]
pub async fn user_search(year: i32, floor: Option<i8>, familyname: Option<String>, givenname: Option<String>, jar: &CookieJar<'_>, app: &State<App>) -> Result<Json<UserSearchResponse>, Status> {

    // Cookieからjwtの取得
    let jwt = match jar.get("token").map(|c| c.value()) {
        None => return Err(Status::BadRequest),
        Some(t) => String::from(t),
    };

    // jwtの検証
    match decode_jwt(&jwt) {
        None => Err(Status::BadRequest),
        Some(_) => {
            // データのバリデーション

            // year
            if year < 2024 {
                return Err(Status::BadRequest);
            }

            // floor
            if let Some(floor) = floor {
                if !(2..=6).contains(&floor) {
                    return Err(Status::BadRequest);
                }
            }

            // familyname
            let family_name_val = match familyname {
                None => String::from(""),
                Some(x) => {
                    let name = String::from(RawStr::new(&x).url_decode().unwrap());
                    let re = Regex::new(r"^[A-Za-z\p{Kana}\p{Hira}\p{Han}]+$").unwrap();
                    if !(re.is_match(name.as_str())) {
                        return Err(Status::BadRequest);
                    }
                    else {
                        name
                    }
                },
            };
            // givenname
            let given_name_val = match givenname {
                None => String::from(""),
                Some(x) => {
                    let name = String::from(RawStr::new(&x).url_decode().unwrap());
                    let re = Regex::new(r"^[A-Za-z\p{Kana}\p{Hira}\p{Han}]+$").unwrap();
                    if !(re.is_match(name.as_str())) {
                        return Err(Status::BadRequest);
                    }
                    else {
                        name
                    }
                },
            };

            let match_user = match app.student.get_by_name(&family_name_val, &given_name_val).await {
                Ok(student) => student,
                Err(_) => return Err(Status::InternalServerError),
            };

            let mut user_pairs= Vec::new();
            for element in match_user {
                match app.student_pair.get_by_id(&element.student_id).await {
                    Ok(Some(student_pair)) => {
                        user_pairs.push(student_pair);
                    },
                    Ok(None) => {},
                    Err(_) => return Err(Status::InternalServerError),
                };
            }

            let unique_user_pair: HashSet<StudentPair> = user_pairs.into_iter().collect();

            let mut matched_record: Vec<AssignmentRecord> = Vec::new();
            for element in unique_user_pair {
                let mut get_result = match app.assignment_record.get(&year, floor, &element.pair_id).await {
                    Ok(res) => res,
                    Err(_) => return Err(Status::InternalServerError),
                };
                matched_record.append(&mut get_result);
            }

            let mut result: Vec<UserSearchResult> = Vec::new();

            for element in matched_record {
                let pair = match app.student_pair.get_by_pair_id(&element.pair_id).await {
                    Ok(studentpair) => studentpair,
                    Err(_) => return Err(Status::InternalServerError),
                };

                let main_user = match app.student.get_by_id(&pair.student_id1).await {
                    Ok(student) => student,
                    Err(_) => return Err(Status::InternalServerError),
                };

                let co_user = match app.student.get_by_id(&pair.student_id2).await {
                    Ok(student) => student,
                    Err(_) => return Err(Status::InternalServerError),
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
                    floor: locker_id_borrow.chars().next().unwrap().to_digit(10).unwrap() as i8,
                    main_user: main_user_info,
                    co_user: co_user_info,
                    year,
                };

                result.push(num);
            }

            result.sort_by(|lt, rt| lt.locker_id.cmp(&rt.locker_id));

            Ok(Json(UserSearchResponse{
                data: result,
            }))
        }
    }
}

/// ロッカーリセットAPI
#[utoipa::path(context_path = "/api/admin/locker")]
#[post("/reset", data = "<request>")]
pub async fn reset(request: Json<LockerResetRequest>, jar: &CookieJar<'_>, app: &State<App>) -> (Status, &'static str) {
    // バリデーション

    // password
    let re = Regex::new(r"^[A-Za-z\d]+$").unwrap();
    if !(re.is_match(request.password.as_str())) {
        return (Status::BadRequest, "request data is not valid");
    }

    // Cookieからjwtの取得
    let jwt = match jar.get("token").map(|c| c.value()) {
        None => return (Status::Unauthorized, "request is unauthorized"),
        Some(t) => String::from(t),
    };

    // jwtの検証
    match decode_jwt(&jwt) {
        None => return (Status::Unauthorized, "request token is not valid"),
        Some(_) => {
            // passwordの検証
            dotenv().ok();
            let password = env::var("LOCKER_RESET_PASSWORD_HASH").expect("locker reset password hash must be set");
            match verify_password_hash(request.password.clone(), password) {
                Ok(_) => {},
                Err(_) => {
                    return (Status::BadRequest, "invalid password")},
            }

            if app.locker.reset_status().await.is_err() {
                return (Status::InternalServerError, "failed to reset locker status")
            };
        }
    }

    (Status::Ok, "successfully reset locker")
}
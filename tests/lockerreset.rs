#[macro_use]
extern crate rocket;
extern crate tus_yuurikai_system;

mod utils;

use std::env;
use utils::{router::rocket, setup::setup_db};
use rocket::local::asynchronous::Client;
use rocket::http::{Status, ContentType, Cookie};
use dotenv::dotenv;
use tus_yuurikai_system::adapters::{controller::locker, httpmodels::LockerResetRequest};
use tus_yuurikai_system::utils::jwt::encode_jwt;
use tus_yuurikai_system::infrastructure::router::{App, AppOption};
use chrono::Duration;


// 正常系
#[rocket::async_test]
#[ignore]
async fn normal() {

    // Arrange
    let client = Client::tracked(rocket()).await.unwrap();
    let app_option = AppOption::new();
    let app = App::new(app_option);

    // dbの初期化
    setup_db(&app).await;

    dotenv().ok();

    let request = LockerResetRequest{
        password: env::var("LOCKER_RESET_PASSWORD").expect("locker reset password must be set"),
    };

    // jwtをCookieに保存
    let username = String::from("test_admin");
    let key = env::var("TOKEN_KEY").expect("token key must be set");
    let token = encode_jwt(&username, Duration::hours(1), &key);
    let cookie = Cookie::build(("token", token))
        .path("/")
        .secure(true)
        .http_only(true);

    match app.locker.locker_repository.update_status_by_id(&String::from("2001"), &String::from("occupied")).await {
        Ok(_) => {},
        Err(err) => {panic!("{}", err)}
    }

    match app.locker.locker_repository.update_status_by_id(&String::from("2002"), &String::from("out-of-work")).await {
        Ok(_) => {},
        Err(err) => {panic!("{}", err)}
    }

    // Act
    let response = client.post(uri!("/api/admin/locker", locker::reset))
        .header(ContentType::JSON)
        .json(&request)
        .cookie(cookie)
        .dispatch().await;

    // Assert
    let status = match app.locker.locker_repository.get_by_id(&String::from("2001")).await {
        Ok(locker) => {locker.status},
        Err(err) => {panic!("{}", err)}
    };
    assert_eq!(status, String::from("vacant"));

    let status = match app.locker.locker_repository.get_by_id(&String::from("2002")).await {
        Ok(locker) => {locker.status},
        Err(err) => {panic!("{}", err)}
    };
    assert_eq!(status, String::from("out-of-work"));

    let status = match app.locker.locker_repository.get_by_id(&String::from("2003")).await {
        Ok(locker) => {locker.status},
        Err(err) => {panic!("{}", err)},
    };
    assert_eq!(status, String::from("vacant"));

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().await.unwrap(), "successfully reset locker");

    setup_db(&app).await;
}

// 正常系:モンキーテスト
#[rocket::async_test]
#[ignore]
async fn out_of_work_locker_exists() {

    // Arrange
    let client = Client::tracked(rocket()).await.unwrap();
    let app_option = AppOption::new();
    let app = App::new(app_option);

    // dbの初期化
    setup_db(&app).await;

    dotenv().ok();

    let request = LockerResetRequest{
        password: env::var("LOCKER_RESET_PASSWORD").expect("locker reset password must be set"),
    };

    // jwtをCookieに保存
    let username = String::from("test_admin");
    let key = env::var("TOKEN_KEY").expect("token key must be set");
    let token = encode_jwt(&username, Duration::hours(1), &key);
    let cookie = Cookie::build(("token", token))
        .path("/")
        .secure(true)
        .http_only(true);

    const VCT: &str = "vacant";
    const OCP: &str = "occupied";
    const OOW: &str = "out-of-work";

    let target_locker = vec!["2001", "3100", "4013", "5031", "6003"];
    let target_status = vec![OCP, OOW, VCT, OCP, OOW];

    for _i in 0..target_locker.len() {
        match app.locker.locker_repository.update_status_by_id(&String::from(target_locker[_i]), &String::from(target_status[_i])).await {
            Ok(_) => {},
            Err(err) => {panic!("{}", err)},
        }
    }

    // Act
    let response = client.post(uri!("/api/admin/locker", locker::reset))
        .header(ContentType::JSON)
        .json(&request)
        .cookie(cookie)
        .dispatch().await;

    // Assert

    for _i in 0..target_locker.len() {
        let status = match app.locker.locker_repository.get_by_id(&String::from(target_locker[_i])).await {
            Ok(locker) => {locker.status},
            Err(err) => {panic!("{}", err)},
        };
        if target_status[_i] == OOW {
            assert_eq!(status, String::from(OOW));
        } else {
            assert_eq!(status, String::from(VCT));
        }
    }

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().await.unwrap(), "successfully reset locker");

    setup_db(&app).await;
}

// 異常系:パスワードが無効
#[rocket::async_test]
#[ignore]
async fn request_password_is_not_valid() {

    // Arrange
    let client = Client::tracked(rocket()).await.unwrap();
    let app_option = AppOption::new();
    let app = App::new(app_option);

    // dbの初期化
    setup_db(&app).await;

    let request = LockerResetRequest{
        password: String::from("testtest"),
    };

    // jwtをCookieに保存
    let username = String::from("test_admin");
    let key = env::var("TOKEN_KEY").expect("token key must be set");
    let token = encode_jwt(&username, Duration::hours(1), &key);
    let cookie = Cookie::build(("token", token))
        .path("/")
        .secure(true)
        .http_only(true);

    match app.locker.locker_repository.update_status_by_id(&String::from("2001"), &String::from("occupied")).await {
        Ok(_) => {},
        Err(err) => {panic!("{}", err)}
    }

    // Act
    let response = client.post(uri!("/api/admin/locker", locker::reset))
        .header(ContentType::JSON)
        .json(&request)
        .cookie(cookie)
        .dispatch().await;

    // Assert
    assert_eq!(response.status(), Status::Unauthorized);
    assert_eq!(response.into_string().await.unwrap(), "request password does not match");

    setup_db(&app).await;
}

// 異常系:jwtが存在しない
#[rocket::async_test]
#[ignore]
async fn jwt_is_not_exists() {

    // Arrange
    let client = Client::tracked(rocket()).await.unwrap();
    let app_option = AppOption::new();
    let app = App::new(app_option);

    // dbの初期化
    setup_db(&app).await;

    let request = LockerResetRequest{
        password: env::var("LOCKER_RESET_PASSWORD").expect("locker reset password must be set"),
    };

    // Act
    let response = client.post(uri!("/api/admin/locker", locker::reset))
        .header(ContentType::JSON)
        .json(&request)
        .dispatch().await;

    // Assert
    assert_eq!(response.status(), Status::Unauthorized);
    assert_eq!(response.into_string().await.unwrap(), "request is unauthorized");

    setup_db(&app).await;
}

// 異常系:jwtが無効
#[rocket::async_test]
#[ignore]
async fn jwt_is_not_valid() {

    // Arrange
    let client = Client::tracked(rocket()).await.unwrap();
    let app_option = AppOption::new();
    let app = App::new(app_option);

    // dbの初期化
    setup_db(&app).await;

    let request = LockerResetRequest{
        password:  env::var("LOCKER_RESET_PASSWORD").expect("locker reset password must be set"),
    };

    // jwtをCookieに保存
    let username = env::var("ADMIN_USER_NAME").expect("admin username must be set");
    let key = env::var("TOKEN_KEY").expect("token key must be set");
    let token = encode_jwt(&username, Duration::minutes(-2), &key);
    let cookie = Cookie::build(("token", token))
        .path("/")
        .secure(true)
        .http_only(true);

    match app.locker.locker_repository.update_status_by_id(&String::from("2001"), &String::from("occupied")).await {
        Ok(_) => {},
        Err(err) => {panic!("{}", err)}
    }

    // Act
    let response = client.post(uri!("/api/admin/locker", locker::reset))
        .header(ContentType::JSON)
        .json(&request)
        .cookie(cookie)
        .dispatch().await;

    // Assert
    assert_eq!(response.status(), Status::Unauthorized);
    assert_eq!(response.into_string().await.unwrap(), "request token is not valid");

    setup_db(&app).await;
}
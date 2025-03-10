#[macro_use]
extern crate rocket;
extern crate tus_yuurikai_system;

mod utils;

use utils::password_hash::compute_password_hash;
use utils::{router::rocket, setup::setup_db};
use rocket::local::asynchronous::Client;
use rocket::http::{Status, ContentType};
use dotenv::dotenv;
use tus_yuurikai_system::adapters::{controller::locker, httpmodels::LoginFormRequest};
use tus_yuurikai_system::infrastructure::router::{App, AppOption};

// 正常系
#[rocket::async_test]
pub async fn normal() {
    // Arrange
    let client = Client::tracked(rocket()).await.unwrap();
    let app_option = AppOption::new();
    let app = App::new(app_option);

    dotenv().ok();

    let request = LoginFormRequest{
        username: String::from("user000"),
        password: String::from("0000"),
    };

    setup_db(&app).await;

    let password_hash = compute_password_hash(request.password.clone()).unwrap();
    match app.admin.admin_repository.insert(&request.username, &password_hash).await {
        Ok(_) => {},
        Err(err) => {panic!{"{}", err}},
    }

    // Act
    let response = client.post(uri!("/api", locker::login))
        .header(ContentType::JSON)
        .json(&request)
        .dispatch().await;

    match app.admin.admin_repository.delete_by_name(&request.username).await {
        Ok(_) => {},
        Err(err) => {panic!{"{}",err}}
    }

    // Assert
    assert_eq!(response.status(), Status::Created);
    assert_ne!(response.cookies().get("token"), None);

    setup_db(&app).await;
}

// 異常系=存在しないusernameである
#[rocket::async_test]
pub async fn username_does_not_exist() {
    // Arrange
    let client = Client::tracked(rocket()).await.unwrap();
    let app_option = AppOption::new();
    let app = App::new(app_option);

    dotenv().ok();

    let correct_username = String::from("user000");

    let request = LoginFormRequest{
        username: String::from("user111"),
        password: String::from("0000"),
    };

    setup_db(&app).await;

    let password_hash = compute_password_hash(request.password.clone()).unwrap();
    match app.admin.admin_repository.insert(&correct_username, &password_hash).await {
        Ok(_) => {},
        Err(err) => {panic!{"{}", err}},
    }

    // Act
    let response = client.post(uri!("/api", locker::login))
        .header(ContentType::JSON)
        .json(&request)
        .dispatch().await;

    match app.admin.admin_repository.delete_by_name(&correct_username).await {
        Ok(_) => {},
        Err(err) => {panic!{"{}",err}}
    }

    // Assert
    assert_eq!(response.status(), Status::InternalServerError);
    assert_eq!(response.cookies().get("token"), None);

    setup_db(&app).await;
}

// 異常系=passwordが異なる
#[rocket::async_test]
pub async fn password_is_wrong() {
    // Arrange
    let client = Client::tracked(rocket()).await.unwrap();
    let app_option = AppOption::new();
    let app = App::new(app_option);

    dotenv().ok();

    let correct_password = String::from("0000");
    let wrong_password = String::from("1111");

    let request = LoginFormRequest{
        username: String::from("user000"),
        password: wrong_password,
    };

    let password_hash = compute_password_hash(correct_password.clone()).unwrap();
    match app.admin.admin_repository.insert(&request.username, &password_hash).await {
        Ok(_) => {},
        Err(err) => {panic!{"{}", err}},
    }

    setup_db(&app).await;

    // Act
    let response = client.post(uri!("/api", locker::login))
        .header(ContentType::JSON)
        .json(&request)
        .dispatch().await;

    match app.admin.admin_repository.delete_by_name(&request.username).await {
        Ok(_) => {},
        Err(err) => {panic!{"{}",err}},
    }

    // Assert
    assert_eq!(response.status(), Status::BadRequest);
    assert_eq!(response.cookies().get("token"), None);

    setup_db(&app).await;
}
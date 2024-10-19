#[macro_use]
extern crate rocket;
extern crate tus_yuurikai_system;

mod utils;

use std::env;
use utils::{router::rocket, setup::setup_db};
use rocket::local::asynchronous::Client;
use rocket::http::{Status, ContentType};
use dotenv::dotenv;
use tus_yuurikai_system::adapters::{controller, httpmodels::LoginFormRequest};
use tus_yuurikai_system::infrastructure::router::App;

// 正常系
#[rocket::async_test]
pub async fn normal() {
    // Arrange
    let client = Client::tracked(rocket()).await.unwrap();
    let app = App::new();

    dotenv().ok();

    let request = LoginFormRequest{
        username: env::var("ADMIN_USER_NAME").expect("admin username must be set"),
        password: env::var("ADMIN_PASSWORD").expect("admin password must be set"),
    };

    setup_db(&app).await;

    // Act
    let response = client.post(uri!("/api", controller::login))
        .header(ContentType::JSON)
        .json(&request)
        .dispatch().await;

    // Assert
    assert_eq!(response.status(), Status::Created);
    assert_ne!(response.cookies().get("token"), None);
}

// 異常系=存在しないusernameである
#[rocket::async_test]
pub async fn username_does_not_exist() {
    // Arrange
    let client = Client::tracked(rocket()).await.unwrap();
    let app = App::new();

    dotenv().ok();

    let request = LoginFormRequest{
        username: String::from("user111"),
        password: env::var("ADMIN_PASSWORD").expect("admin password must be set"),
    };

    setup_db(&app).await;

    // Act
    let response = client.post(uri!("/api", controller::login))
        .header(ContentType::JSON)
        .json(&request)
        .dispatch().await;

    // Assert
    assert_eq!(response.status(), Status::InternalServerError);
    assert_eq!(response.cookies().get("token"), None);
}

// 異常系=passwordが異なる
#[rocket::async_test]
pub async fn password_is_wrong() {
    // Arrange
    let client = Client::tracked(rocket()).await.unwrap();
    let app = App::new();

    dotenv().ok();

    let request = LoginFormRequest{
        username: env::var("ADMIN_USER_NAME").expect("admin username must be set"),
        password: String::from("1111"),
    };

    setup_db(&app).await;

    // Act
    let response = client.post(uri!("/api", controller::login))
        .header(ContentType::JSON)
        .json(&request)
        .dispatch().await;

    // Assert
    assert_eq!(response.status(), Status::InternalServerError);
    assert_eq!(response.cookies().get("token"), None);
}
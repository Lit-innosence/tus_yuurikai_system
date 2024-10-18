#[macro_use]
extern crate rocket;
extern crate tus_yuurikai_system;

mod utils;

use utils::{router::rocket, setup::setup_db};
use rocket::local::asynchronous::Client;
use rocket::http::{Status, ContentType};
use tus_yuurikai_system::adapters::controller::{self, LoginFormRequest};
use tus_yuurikai_system::infrastructure::router::App;

// 正常系
#[rocket::async_test]
pub async fn normal() {
    // Arrange
    let client = Client::tracked(rocket()).await.unwrap();
    let app = App::new();

    let request = LoginFormRequest{
        username: String::from("user000"),
        password: String::from("0000"),
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

    let request = LoginFormRequest{
        username: String::from("user111"),
        password: String::from("0000"),
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

    let request = LoginFormRequest{
        username: String::from("user000"),
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
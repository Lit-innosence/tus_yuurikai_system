#[macro_use]
extern crate rocket;
extern crate tus_yuurikai_system;

mod utils;

use tus_yuurikai_system::infrastructure::router::{App, AppOption};
use utils::{router::rocket, setup::setup_db};
use rocket::local::asynchronous::Client;
use rocket::http::{Status, ContentType};
use tus_yuurikai_system::adapters::{controller, httpmodels::HealthCheckRequest};

#[rocket::async_test]
async fn get_healthcheck_test() {
    // Arrange
    let client = Client::tracked(rocket()).await.unwrap();

    // Act
    let response = client.get(uri!("/api", controller::get_healthcheck)).dispatch().await;

    // Assert
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().await.unwrap(), "Hello, world!");
}

#[rocket::async_test]
async fn post_healthcheck_test() {
    // Arrange
    let client = Client::tracked(rocket()).await.unwrap();
    let request = HealthCheckRequest{
        text: String::from("Hello world from json!")
    };

    let app_option = AppOption::new();
    let app = App::new(app_option);

    setup_db(&app).await;

    // Act
    let response = client.post(uri!("/api", controller::post_healthcheck))
        .header(ContentType::JSON)
        .json(&request)
        .dispatch().await;

    // Assert
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().await.unwrap(), "Accepted post request! \"Hello world from json!\"")
}
extern crate rocket;
extern crate tus_yuurikai_system;

mod utils;

use utils::{router::rocket, setup::setup_db};
use rocket::local::asynchronous::Client;
use rocket::http::Status;
use tus_yuurikai_system::adapters::httpmodels::{LockerStatus, LockerStatusResponse};
use tus_yuurikai_system::infrastructure::router::App;

// 正常系
#[rocket::async_test]
pub async fn normal() {
    // Arrange
    let client = Client::tracked(rocket()).await.unwrap();
    let app = App::new();

    setup_db(&app).await;

    let mut results: Vec<LockerStatus> = Vec::new();
    let result = app.locker.locker_repository.get_by_floor(&String::from("")).await.unwrap();
    for element in result {
        let data = LockerStatus{
            locker_id: element.locker_id.clone(),
            floor: element.locker_id.chars().nth(0).unwrap().to_digit(10).unwrap() as i8,
            status: element.status,
        };
        results.push(data);
    }

    let expected_result = LockerStatusResponse{
        data: results
    };

    // Act
    let response = client.get("/api/locker/availability").dispatch().await;

    // Assert
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_json::<LockerStatusResponse>().await.unwrap(), expected_result);
}

// 正常系=floorが指定されていない
#[rocket::async_test]
pub async fn floor_is_not_requested() {
    // Arrange
    let client = Client::tracked(rocket()).await.unwrap();
    let app = App::new();

    setup_db(&app).await;

    let mut results: Vec<LockerStatus> = Vec::new();
    let result = app.locker.locker_repository.get_by_floor(&String::from("")).await.unwrap();
    for element in result {
        let data = LockerStatus{
            locker_id: element.locker_id.clone(),
            floor: element.locker_id.chars().nth(0).unwrap().to_digit(10).unwrap() as i8,
            status: element.status,
        };
        results.push(data);
    }

    let expected_result = LockerStatusResponse{
        data: results
    };

    // Act
    let response = client.get("/api/locker/availability").dispatch().await;

    // Assert
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_json::<LockerStatusResponse>().await.unwrap(), expected_result);
}
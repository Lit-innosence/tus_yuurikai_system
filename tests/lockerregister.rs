#[macro_use]
extern crate rocket;
extern crate tus_yuurikai_system;

mod utils;

use rocket::response;
use utils::router::rocket;
use rocket::local::asynchronous::Client;
use rocket::http::{Status, ContentType};
use tus_yuurikai_system::adapters::controller::{self, LockerResisterRequest};
use tus_yuurikai_system::domain::{assignment::AssignmentInfo, student_pair::PairInfo, student::UserInfo};
use tus_yuurikai_system::usecase::{student_pair::StudentPairUsecase, student::StudentUsecase};
use tus_yuurikai_system::infrastracture::router::App;

#[rocket::async_test]
async fn passing() {
    // Arrange
    let client = Client::tracked(rocket()).await.unwrap();
    let app = App::new();
    let request = LockerResisterRequest{
        data: AssignmentInfo{
            student_id: String::from("4622999"),
            locker_id: String::from("2001"),
        }
    };

    let mainuser = &UserInfo{
            student_id: String::from("4622999"),
            family_name: String::from("test_user"),
            given_name: String::from("test_user")
        };
    let couser = &UserInfo{
            student_id: String::from("4622000"),
            family_name: String::from("test_user"),
            given_name: String::from("test_user")
        };

    let studentpair = &PairInfo{
        main_user: mainuser.clone(),
        co_user: couser.clone()
    };
    // student2人をdbに保存
    match app.student.register(mainuser).await {
        Ok(_) => {},
        Err(err) => {panic!("{}", err);},
    };
    match app.student.register(couser).await {
        Ok(_) => {},
        Err(err) => {panic!("{}", err);},
    };
    // studentpairをdbに保存
    match app.student_pair.register(studentpair).await {
        Ok(_) => {},
        Err(err) => {panic!("{}", err);},
    };

    // Act
    let response = client.post(uri!(controller::locker_register))
        .header(ContentType::JSON)
        .json(&request)
        .dispatch().await;

    // Assert
    assert_eq!(response.status(), Status::Created);
    assert_eq!(response.into_string().await.unwrap(), "success create assignment");
}
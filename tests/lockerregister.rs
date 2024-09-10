#[macro_use]
extern crate rocket;
extern crate tus_yuurikai_system;

mod utils;

use utils::{router::rocket, setup::setup_db};
use rocket::local::asynchronous::Client;
use rocket::http::{Status, ContentType};
use tus_yuurikai_system::adapters::controller::{self, LockerResisterRequest};
use tus_yuurikai_system::domain::{assignment::AssignmentInfo, student_pair::PairInfo, student::UserInfo};
use tus_yuurikai_system::usecase::{student_pair::StudentPairUsecase, student::StudentUsecase};
use tus_yuurikai_system::infrastracture::router::App;

// 正常系
#[rocket::async_test]
async fn normal() {

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

    // dbの初期化
    setup_db();

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

// 正常系＝学籍番号にA,Bを許す
#[rocket::async_test]
async fn student_id_allow_a_b() {

    // Arrange
    let client = Client::tracked(rocket()).await.unwrap();
    let app = App::new();
    let request = LockerResisterRequest{
        data: AssignmentInfo{
            student_id: String::from("3A22999"),
            locker_id: String::from("2001"),
        }
    };

    let mainuser = &UserInfo{
            student_id: String::from("3A22999"),
            family_name: String::from("test_user"),
            given_name: String::from("test_user")
        };
    let couser = &UserInfo{
            student_id: String::from("3B22999"),
            family_name: String::from("test_user"),
            given_name: String::from("test_user")
        };

    let studentpair = &PairInfo{
        main_user: mainuser.clone(),
        co_user: couser.clone()
    };

    // dbの初期化
    setup_db();

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

// 異常系＝student_idがテーブル内のタプルと一致しない
#[rocket::async_test]
async fn student_id_do_not_match() {

    // Arrange
    let client = Client::tracked(rocket()).await.unwrap();
    let app = App::new();
    let request = LockerResisterRequest{
        data: AssignmentInfo{
            student_id: String::from("4622000"),
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

    // dbの初期化
    setup_db();

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
    assert_eq!(response.status(), Status::InternalServerError);
    assert_eq!(response.into_string().await.unwrap(), "failed to get student_pair id");
}


// 異常系＝yearが一致するタプルが存在しない
#[rocket::async_test]
async fn year_do_not_match() {

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
    /*
    let studentpair = &PairInfo{
        main_user: mainuser.clone(),
        co_user: couser.clone()
    };
    */

    // dbの初期化
    setup_db();

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
    let year = 2000;
    match app.student_pair.student_pair_repository.insert(&mainuser.student_id, &couser.student_id, &year).await {
        Ok(_) => {},
        Err(err) => {panic!("{}", err);},
    };


    // Act
    let response = client.post(uri!(controller::locker_register))
        .header(ContentType::JSON)
        .json(&request)
        .dispatch().await;

    // Assert
    assert_eq!(response.status(), Status::InternalServerError);
    assert_eq!(response.into_string().await.unwrap(), "failed to get student_pair id");
}
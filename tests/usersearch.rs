extern crate rocket;
extern crate tus_yuurikai_system;

mod utils;

use utils::{router::rocket, setup::setup_db};
use rocket::local::asynchronous::Client;
use rocket::http::Status;
use tus_yuurikai_system::adapters::controller::{UserSearchResult, UserSearchResponce};
use tus_yuurikai_system::domain::{assignment::AssignmentInfo, student_pair::PairInfo, student::UserInfo};
use tus_yuurikai_system::usecase::{assignment_record::AssignmentRecordUsecase, student_pair::StudentPairUsecase, student::StudentUsecase};
use tus_yuurikai_system::infrastracture::router::App;
use chrono::{Datelike, Local};

// 正常系
#[rocket::async_test]
async fn normal() {
    //Arrange
    let client = Client::tracked(rocket()).await.unwrap();
    let app = App::new();

    let mainuser = &UserInfo{
            student_id: String::from("4622999"),
            family_name: String::from("テスト"),
            given_name: String::from("太郎")
        };
    let couser = &UserInfo{
            student_id: String::from("4622000"),
            family_name: String::from("テスト"),
            given_name: String::from("次郎")
        };

    let studentpair = &PairInfo{
        main_user: mainuser.clone(),
        co_user: couser.clone()
    };

    let assignment = &AssignmentInfo{
        student_id: String::from("4622999"),
        locker_id: String::from("2001"),
    };

    // dbの初期化
    setup_db(&app).await;

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
    let student_pair = match app.student_pair.register(studentpair).await {
        Ok(student_pair) => student_pair,
        Err(err) => {panic!("{}", err);},
    };

    match app.assignment_record.register(&student_pair, assignment).await {
        Ok(_) => {},
        Err(err) => {panic!("{}", err);},
    }

    // Act
    let response = client.get("/api/locker/user-search/2024?familyname=%E3%83%86%E3%82%B9%E3%83%88&givenname=%E5%A4%AA%E9%83%8E").dispatch().await;

    let expected_data = UserSearchResponce{
        data : vec![
        UserSearchResult {
            locker_id: String::from("2001"),
            floor: 2,
            main_user: mainuser.clone(),
            co_user: couser.clone(),
            year: Local::now().year(),
        }]
    };

    // Assert
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_json::<UserSearchResponce>().await.unwrap(), expected_data);
}

// 正常系:given_nameが指定されていない
#[rocket::async_test]
async fn given_name_is_not_requested() {
    //Arrange
    let client = Client::tracked(rocket()).await.unwrap();
    let app = App::new();

    let mainuser = &UserInfo{
            student_id: String::from("4622999"),
            family_name: String::from("テスト"),
            given_name: String::from("太郎")
        };
    let couser = &UserInfo{
            student_id: String::from("4622000"),
            family_name: String::from("テスト"),
            given_name: String::from("次郎")
        };

    let studentpair = &PairInfo{
        main_user: mainuser.clone(),
        co_user: couser.clone()
    };

    let assignment = &AssignmentInfo{
        student_id: String::from("4622999"),
        locker_id: String::from("2001"),
    };

    // dbの初期化
    setup_db(&app).await;

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
    let student_pair = match app.student_pair.register(studentpair).await {
        Ok(student_pair) => student_pair,
        Err(err) => {panic!("{}", err);},
    };

    match app.assignment_record.register(&student_pair, assignment).await {
        Ok(_) => {},
        Err(err) => {panic!("{}", err);},
    }

    // Act
    let response = client.get("/api/locker/user-search/2024?familyname=%E3%83%86%E3%82%B9%E3%83%88").dispatch().await;

    let expected_data = UserSearchResponce{
        data : vec![
        UserSearchResult {
            locker_id: String::from("2001"),
            floor: 2,
            main_user: mainuser.clone(),
            co_user: couser.clone(),
            year: Local::now().year(),
        }]
    };

    // Assert
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_json::<UserSearchResponce>().await.unwrap(), expected_data);
}

// 正常系:family_nameが指定されていない
#[rocket::async_test]
async fn family_name_is_not_requested() {
    //Arrange
    let client = Client::tracked(rocket()).await.unwrap();
    let app = App::new();

    let mainuser = &UserInfo{
            student_id: String::from("4622999"),
            family_name: String::from("テスト"),
            given_name: String::from("太郎")
        };
    let couser = &UserInfo{
            student_id: String::from("4622000"),
            family_name: String::from("テスト"),
            given_name: String::from("次郎")
        };

    let studentpair = &PairInfo{
        main_user: mainuser.clone(),
        co_user: couser.clone()
    };

    let assignment = &AssignmentInfo{
        student_id: String::from("4622999"),
        locker_id: String::from("2001"),
    };

    // dbの初期化
    setup_db(&app).await;

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
    let student_pair = match app.student_pair.register(studentpair).await {
        Ok(student_pair) => student_pair,
        Err(err) => {panic!("{}", err);},
    };

    match app.assignment_record.register(&student_pair, assignment).await {
        Ok(_) => {},
        Err(err) => {panic!("{}", err);},
    }

    // Act
    let response = client.get("/api/locker/user-search/2024?givenname=%E5%A4%AA%E9%83%8E").dispatch().await;

    let expected_data = UserSearchResponce{
        data : vec![
        UserSearchResult {
            locker_id: String::from("2001"),
            floor: 2,
            main_user: mainuser.clone(),
            co_user: couser.clone(),
            year: Local::now().year(),
        }]
    };

    // Assert
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_json::<UserSearchResponce>().await.unwrap(), expected_data);
}

// 正常系:nameが指定されていない
#[rocket::async_test]
async fn name_is_not_requested() {
    //Arrange
    let client = Client::tracked(rocket()).await.unwrap();
    let app = App::new();

    let mainuser = &UserInfo{
            student_id: String::from("4622999"),
            family_name: String::from("テスト"),
            given_name: String::from("太郎")
        };
    let couser = &UserInfo{
            student_id: String::from("4622000"),
            family_name: String::from("テスト"),
            given_name: String::from("次郎")
        };

    let studentpair = &PairInfo{
        main_user: mainuser.clone(),
        co_user: couser.clone()
    };

    let assignment = &AssignmentInfo{
        student_id: String::from("4622999"),
        locker_id: String::from("2001"),
    };

    // dbの初期化
    setup_db(&app).await;

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
    let student_pair = match app.student_pair.register(studentpair).await {
        Ok(student_pair) => student_pair,
        Err(err) => {panic!("{}", err);},
    };

    match app.assignment_record.register(&student_pair, assignment).await {
        Ok(_) => {},
        Err(err) => {panic!("{}", err);},
    }

    // Act
    let response = client.get("/api/locker/user-search/2024").dispatch().await;

    let expected_data = UserSearchResponce{
        data : vec![
        UserSearchResult {
            locker_id: String::from("2001"),
            floor: 2,
            main_user: mainuser.clone(),
            co_user: couser.clone(),
            year: Local::now().year(),
        }]
    };

    // Assert
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_json::<UserSearchResponce>().await.unwrap(), expected_data);
}
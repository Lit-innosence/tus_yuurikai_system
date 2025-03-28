#[macro_use]
extern crate rocket;
extern crate tus_yuurikai_system;

mod utils;

use utils::{router::rocket, setup::setup_db};
use rocket::local::asynchronous::Client;
use rocket::http::{Status, ContentType};
use tus_yuurikai_system::adapters::{controller::locker, httpmodels::LockerResisterRequest};
use tus_yuurikai_system::domain::{assignment::AssignmentInfo, student_pair::PairInfo, student::UserInfo};
use tus_yuurikai_system::usecase::{student_pair::StudentPairUsecase, student::StudentUsecase, auth::AuthUsecase};
use tus_yuurikai_system::infrastructure::router::{App, AppOption};

// 正常系
#[rocket::async_test]
#[ignore]
async fn normal() {

    // Arrange
    let client = Client::tracked(rocket()).await.unwrap();
    let app_option = AppOption::new();
    let app = App::new(app_option);

    let mainuser = &UserInfo{
            student_id: String::from("4622999"),
            family_name: String::from("テスト太郎"),
            given_name: String::from("てすと太郎")
        };
    let couser = &UserInfo{
            student_id: String::from("4622000"),
            family_name: String::from("テスト太郎"),
            given_name: String::from("てすと太郎")
        };

    let studentpair = &PairInfo{
        main_user: mainuser.clone(),
        co_user: couser.clone()
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
    match app.student_pair.register(studentpair).await {
        Ok(_) => {},
        Err(err) => {panic!("{}", err);},
    };

    // 認証完了用のレコードを保存
    let auth_id = match app.auth.locker_register(mainuser, couser, &String::from("auth_check"), true).await{
        Ok(auth) => auth.auth_id,
        Err(err) => {panic!("{}", err)},
    };

    let request = LockerResisterRequest{
        data: AssignmentInfo{
            student_id: String::from("4622999"),
            locker_id: String::from("2001"),
        },
        auth_id: auth_id.to_string(),
    };


    // Act
    let response = client.post(uri!("/api/locker", locker::locker_register))
        .header(ContentType::JSON)
        .json(&request)
        .dispatch().await;

    // Assert
    assert_eq!(response.status(), Status::Created);
    assert_eq!(response.into_string().await.unwrap(), "success create assignment");

    setup_db(&app).await;
}

// 正常系＝学籍番号にA,Bを許す
#[rocket::async_test]
#[ignore]
async fn student_id_allow_a_b() {

    // Arrange
    let client = Client::tracked(rocket()).await.unwrap();
    let app_option = AppOption::new();
    let app = App::new(app_option);

    let mainuser = &UserInfo{
            student_id: String::from("3A22999"),
            family_name: String::from("テスト太郎"),
            given_name: String::from("てすと太郎")
        };
    let couser = &UserInfo{
            student_id: String::from("3B22999"),
            family_name: String::from("テスト太郎"),
            given_name: String::from("てすと太郎")
        };

    let studentpair = &PairInfo{
        main_user: mainuser.clone(),
        co_user: couser.clone()
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
    match app.student_pair.register(studentpair).await {
        Ok(_) => {},
        Err(err) => {panic!("{}", err);},
    };

    // 認証完了用のレコードを保存
    let auth_id = match app.auth.locker_register(mainuser, couser, &String::from("auth_check"), true).await{
        Ok(auth) => auth.auth_id,
        Err(err) => {panic!("{}", err)},
    };

    let request = LockerResisterRequest{
        data: AssignmentInfo{
            student_id: String::from("3A22999"),
            locker_id: String::from("2001"),
        },
        auth_id: auth_id.to_string(),
    };


    // Act
    let response = client.post(uri!("/api/locker", locker::locker_register))
        .header(ContentType::JSON)
        .json(&request)
        .dispatch().await;

    // Assert
    assert_eq!(response.status(), Status::Created);
    assert_eq!(response.into_string().await.unwrap(), "success create assignment");

    setup_db(&app).await;
}

// 異常系＝student_idがテーブル内のタプルと一致しない
#[rocket::async_test]
#[ignore]
async fn student_id_do_not_match() {

    // Arrange
    let client = Client::tracked(rocket()).await.unwrap();
    let app_option = AppOption::new();
    let app = App::new(app_option);

    let mainuser = &UserInfo{
            student_id: String::from("4622999"),
            family_name: String::from("テスト太郎"),
            given_name: String::from("てすと太郎")
        };
    let couser = &UserInfo{
            student_id: String::from("4622000"),
            family_name: String::from("テスト太郎"),
            given_name: String::from("てすと太郎")
        };

    let studentpair = &PairInfo{
        main_user: mainuser.clone(),
        co_user: couser.clone()
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
    match app.student_pair.register(studentpair).await {
        Ok(_) => {},
        Err(err) => {panic!("{}", err);},
    };

    // 認証完了用のレコードを保存
    let auth_id = match app.auth.locker_register(mainuser, couser, &String::from("auth_check"), true).await{
        Ok(auth) => auth.auth_id,
        Err(err) => {panic!("{}", err)},
    };

    let request = LockerResisterRequest{
        data: AssignmentInfo{
            student_id: String::from("4622000"),
            locker_id: String::from("2001"),
        },
        auth_id: auth_id.to_string(),
    };

    // Act
    let response = client.post(uri!("/api/locker", locker::locker_register))
        .header(ContentType::JSON)
        .json(&request)
        .dispatch().await;

    // Assert
    assert_eq!(response.status(), Status::InternalServerError);
    assert_eq!(response.into_string().await.unwrap(), "failed to get student_pair id");

    setup_db(&app).await;
}


// 異常系＝yearが一致するタプルが存在しない
#[rocket::async_test]
#[ignore]
async fn year_do_not_match() {

    // Arrange
    let client = Client::tracked(rocket()).await.unwrap();
    let app_option = AppOption::new();
    let app = App::new(app_option);

    let mainuser = &UserInfo{
            student_id: String::from("4622999"),
            family_name: String::from("テスト太郎"),
            given_name: String::from("てすと太郎")
        };
    let couser = &UserInfo{
            student_id: String::from("4622000"),
            family_name: String::from("テスト太郎"),
            given_name: String::from("てすと太郎")
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

    // yearが一致しないstudentpairをdbに保存
    let year = 2000;
    match app.student_pair.student_pair_repository.insert(&mainuser.student_id, &couser.student_id, &year).await {
        Ok(_) => {},
        Err(err) => {panic!("{}", err);},
    };

    // 認証完了用のレコードを保存
    let auth_id = match app.auth.locker_register(mainuser, couser, &String::from("auth_check"), true).await{
        Ok(auth) => auth.auth_id,
        Err(err) => {panic!("{}", err)},
    };

    let request = LockerResisterRequest{
        data: AssignmentInfo{
            student_id: String::from("4622999"),
            locker_id: String::from("2001"),
        },
        auth_id: auth_id.to_string(),
    };

    // Act
    let response = client.post(uri!("/api/locker", locker::locker_register))
        .header(ContentType::JSON)
        .json(&request)
        .dispatch().await;

    // Assert
    assert_eq!(response.status(), Status::InternalServerError);
    assert_eq!(response.into_string().await.unwrap(), "failed to get student_pair id");

    setup_db(&app).await;
}

// 異常系＝ロッカーのstatusがvacantでない
#[rocket::async_test]
#[ignore]
async fn locker_status_unavailable() {

    // Arrange
    let client = Client::tracked(rocket()).await.unwrap();
    let app_option = AppOption::new();
    let app = App::new(app_option);

    let mainuser = &UserInfo{
            student_id: String::from("4622999"),
            family_name: String::from("テスト太郎"),
            given_name: String::from("てすと太郎")
        };
    let couser = &UserInfo{
            student_id: String::from("4622000"),
            family_name: String::from("テスト太郎"),
            given_name: String::from("てすと太郎")
        };
    let studentpair = &PairInfo{
        main_user: mainuser.clone(),
        co_user: couser.clone()
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
    match app.student_pair.register(studentpair).await {
        Ok(_) => {},
        Err(err) => {panic!("{}", err);},
    };

    // 認証完了用のレコードを保存
    let auth_id = match app.auth.locker_register(mainuser, couser, &String::from("auth_check"), true).await{
        Ok(auth) => auth.auth_id,
        Err(err) => {panic!("{}", err)},
    };

    let request = LockerResisterRequest{
        data: AssignmentInfo{
            student_id: String::from("4622999"),
            locker_id: String::from("2001"),
        },
        auth_id: auth_id.to_string(),
    };

    // 該当lockerのstatusをunavailableに変更
    match app.locker.locker_repository.update_status_by_id(&request.data.locker_id, &String::from("unavailable")).await {
        Ok(_) => {},
        Err(err) => {panic!("{}", err);},
    };


    // Act
    let response = client.post(uri!("/api/locker", locker::locker_register))
        .header(ContentType::JSON)
        .json(&request)
        .dispatch().await;

    // Assert
    assert_eq!(response.status(), Status::BadRequest);
    assert_eq!(response.into_string().await.unwrap(), "This locker is not vacant");

    setup_db(&app).await;
}

// 異常系:既に登録されたペアである
#[rocket::async_test]
#[ignore]
async fn same_pair_arleady_registered() {

    // Arrange
    let client = Client::tracked(rocket()).await.unwrap();
    let app_option = AppOption::new();
    let app = App::new(app_option);

    let mainuser = &UserInfo{
            student_id: String::from("4622999"),
            family_name: String::from("テスト太郎"),
            given_name: String::from("てすと太郎")
        };
    let couser = &UserInfo{
            student_id: String::from("4622000"),
            family_name: String::from("テスト太郎"),
            given_name: String::from("てすと太郎")
        };

    let studentpair = &PairInfo{
        main_user: mainuser.clone(),
        co_user: couser.clone()
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
    match app.student_pair.register(studentpair).await {
        Ok(_) => {},
        Err(err) => {panic!("{}", err);},
    };

    // 認証完了用のレコードを保存
    let auth_id = match app.auth.locker_register(mainuser, couser, &String::from("auth_check"), true).await{
        Ok(auth) => auth.auth_id,
        Err(err) => {panic!("{}", err)},
    };

    let request = LockerResisterRequest{
        data: AssignmentInfo{
            student_id: String::from("4622999"),
            locker_id: String::from("2001"),
        },
        auth_id: auth_id.to_string(),
    };

    // Act
    let _response = client.post(uri!("/api/locker", locker::locker_register))
        .header(ContentType::JSON)
        .json(&request)
        .dispatch().await;

    let response = client.post(uri!("/api/locker", locker::locker_register))
        .header(ContentType::JSON)
        .json(&request)
        .dispatch().await;


    // Assert
    assert_eq!(response.status(), Status::InternalServerError);
    assert_eq!(response.into_string().await.unwrap(), "same pair already exists");

    setup_db(&app).await;
}
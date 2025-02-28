extern crate rocket;
extern crate tus_yuurikai_system;

mod utils;

use std::env;
use utils::{router::rocket, setup::setup_db};
use rocket::local::asynchronous::Client;
use rocket::http::{Status, Cookie};
use dotenv::dotenv;
use tus_yuurikai_system::adapters::httpmodels::{UserSearchResponse, UserSearchResult};
use tus_yuurikai_system::domain::{assignment::AssignmentInfo, student_pair::PairInfo, student::UserInfo};
use tus_yuurikai_system::usecase::{assignment_record::AssignmentRecordUsecase, student_pair::StudentPairUsecase, student::StudentUsecase};
use tus_yuurikai_system::infrastructure::router::App;
use tus_yuurikai_system::utils::jwt::encode_jwt;
use chrono::{Datelike, Local, Duration};

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

    dotenv().ok();

    let username = String::from("user000");
    let password = String::from("0000");

    let password_hash = utils::password_hash::compute_password_hash(password.clone()).unwrap();
    match app.admin.admin_repository.insert(&username, &password_hash).await {
        Ok(_) => {},
        Err(err) => {panic!{"{}", err}},
    }

    let key = env::var("TOKEN_KEY").expect("token key must be set");
    let token = encode_jwt(&username, Duration::hours(1), &key);
    let cookie = Cookie::build(("token", token))
        .path("/")
        .secure(true)
        .http_only(true);

    let year = Local::now().year();

    // Act
    let response = client.get(format!("/api/admin/locker/user-search/{}?familyname=%E3%83%86%E3%82%B9%E3%83%88&givenname=%E5%A4%AA%E9%83%8E", year))
        .cookie(cookie)
        .dispatch().await;

    let expected_data = UserSearchResponse{
        data : vec![
        UserSearchResult {
            locker_id: String::from("2001"),
            floor: 2,
            main_user: mainuser.clone(),
            co_user: couser.clone(),
            year: Local::now().year(),
        }]
    };

    match app.admin.admin_repository.delete_by_name(&username).await {
        Ok(_) => {},
        Err(err) => {panic!{"{}",err}}
    }

    // Assert
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_json::<UserSearchResponse>().await.unwrap(), expected_data);

    setup_db(&app).await;
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

    dotenv().ok();

    let username = String::from("user000");
    let password = String::from("0000");

    let password_hash = utils::password_hash::compute_password_hash(password.clone()).unwrap();
    match app.admin.admin_repository.insert(&username, &password_hash).await {
        Ok(_) => {},
        Err(err) => {panic!{"{}", err}},
    }

    let key = env::var("TOKEN_KEY").expect("token key must be set");
    let token = encode_jwt(&username, Duration::hours(1), &key);
    let cookie = Cookie::build(("token", token))
        .path("/")
        .secure(true)
        .http_only(true);

    let year = Local::now().year();


    // Act
    let response = client.get(format!("/api/admin/locker/user-search/{}?familyname=%E3%83%86%E3%82%B9%E3%83%88&givenname=%E5%A4%AA%E9%83%8E", year))
        .cookie(cookie)
        .dispatch().await;

    let expected_data = UserSearchResponse{
        data : vec![
        UserSearchResult {
            locker_id: String::from("2001"),
            floor: 2,
            main_user: mainuser.clone(),
            co_user: couser.clone(),
            year: Local::now().year(),
        }]
    };

    match app.admin.admin_repository.delete_by_name(&username).await {
        Ok(_) => {},
        Err(err) => {panic!{"{}",err}}
    }

    // Assert
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_json::<UserSearchResponse>().await.unwrap(), expected_data);

    setup_db(&app).await;
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

    dotenv().ok();

    let username = String::from("user000");
    let password = String::from("0000");

    let password_hash = utils::password_hash::compute_password_hash(password.clone()).unwrap();
    match app.admin.admin_repository.insert(&username, &password_hash).await {
        Ok(_) => {},
        Err(err) => {panic!{"{}", err}},
    }

    let key = env::var("TOKEN_KEY").expect("token key must be set");
    let token = encode_jwt(&username, Duration::hours(1), &key);
    let cookie = Cookie::build(("token", token))
        .path("/")
        .secure(true)
        .http_only(true);

    let year = Local::now().year();

    // Act
    let response = client.get(format!("/api/admin/locker/user-search/{}?familyname=%E3%83%86%E3%82%B9%E3%83%88&givenname=%E5%A4%AA%E9%83%8E", year))
        .cookie(cookie)
        .dispatch().await;

    let expected_data = UserSearchResponse{
        data : vec![
        UserSearchResult {
            locker_id: String::from("2001"),
            floor: 2,
            main_user: mainuser.clone(),
            co_user: couser.clone(),
            year: Local::now().year(),
        }]
    };

    match app.admin.admin_repository.delete_by_name(&username).await {
        Ok(_) => {},
        Err(err) => {panic!{"{}",err}}
    }

    // Assert
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_json::<UserSearchResponse>().await.unwrap(), expected_data);

    setup_db(&app).await;
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

    dotenv().ok();

    let username = String::from("user000");
    let password = String::from("0000");

    let password_hash = utils::password_hash::compute_password_hash(password.clone()).unwrap();
    match app.admin.admin_repository.insert(&username, &password_hash).await {
        Ok(_) => {},
        Err(err) => {panic!{"{}", err}},
    }

    let key = env::var("TOKEN_KEY").expect("token key must be set");
    let token = encode_jwt(&username, Duration::hours(1), &key);
    let cookie = Cookie::build(("token", token))
        .path("/")
        .secure(true)
        .http_only(true);

    let year = Local::now().year();

    // Act
    let response = client.get(format!("/api/admin/locker/user-search/{}?familyname=%E3%83%86%E3%82%B9%E3%83%88&givenname=%E5%A4%AA%E9%83%8E", year))
        .cookie(cookie)
        .dispatch().await;

    let expected_data = UserSearchResponse{
        data : vec![
        UserSearchResult {
            locker_id: String::from("2001"),
            floor: 2,
            main_user: mainuser.clone(),
            co_user: couser.clone(),
            year: Local::now().year(),
        }]
    };

    match app.admin.admin_repository.delete_by_name(&username).await {
        Ok(_) => {},
        Err(err) => {panic!{"{}",err}}
    }

    // Assert
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_json::<UserSearchResponse>().await.unwrap(), expected_data);

    setup_db(&app).await;
}

// 異常系:jwtを所持していない
#[rocket::async_test]
async fn jwt_does_not_exist() {
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

    dotenv().ok();

    let year = Local::now().year();

    // Act
    let response = client.get(format!("/api/admin/locker/user-search/{}?familyname=%E3%83%86%E3%82%B9%E3%83%88&givenname=%E5%A4%AA%E9%83%8E", year)).dispatch().await;

    // Assert
    assert_eq!(response.status(), Status::BadRequest);

    setup_db(&app).await;
}
pub mod circle;
pub mod locker;

use crate::adapters::httpmodels::{HealthCheckRequest, DownloadRequest, DownloadResponse};
use crate::adapters::controller::{locker::*, circle::*};
use crate::adapters::httpmodels::*;
use crate::domain::{student::{UserInfo, RepresentativeInfo}, student_pair::PairInfo, assignment::AssignmentInfo, circle::{OrganizationInfo, Organization, OrganizationUpdateInfo}};
use crate::infrastructure::router::App;
use crate::usecase::{
    student::StudentUsecase,
    student_pair::StudentPairUsecase,
    assignment_record::AssignmentRecordUsecase,
    locker::LockerUsecase,
    representatives::RepresentativesUsecase,
    organization::OrganizationUsecase,
    registration::RegistrationUsecase,
    time::TimeUsecase,
};
use crate::utils::{jwt::decode_jwt, verify_password::verify_password_hash};
use rocket::{get, post, serde::json::Json, State, http::{Status, CookieJar}};
use utoipa::OpenApi;
use regex::Regex;
use dotenv::dotenv;
use std::{env, io::{Cursor, Write}};
use chrono::Utc;
use csv::Writer;
use zip::{write::{FileOptions, ExtendedFileOptions}, CompressionMethod, ZipWriter};

#[derive(OpenApi)]
#[openapi(
    paths(
        get_healthcheck,
        post_healthcheck,
        download,
        token_generator,
        main_auth,
        co_auth,
        auth_check,
        locker_register,
        login,
        logout,
        user_search,
        availability,
        reset,
        update_entry,
        update_token_generator,
        register_token_generator,
        circle_main_auth,
        circle_co_auth,
        circle_status,
        access_setting_post,
        access_setting_get,
        circle_list,
        circle_status_update,
    ),
    components(schemas(
        HealthCheckRequest,
        UserInfo,
        PairInfo,
        LockerTokenGenRequest,
        AuthCheckResponse,
        AssignmentInfo,
        LockerResisterRequest,
        LoginFormRequest,
        LockerStatusResponse,
        UserSearchResponse,
        LockerResetRequest,
        DownloadRequest,
        DownloadResponse,
        RepresentativeInfo,
        Organization,
        OrganizationInfo,
        OrganizationUpdateInfo,
        OrganizationStatusUpdateRequest,
        CircleUpdateRequest,
        CircleTokenGenRequest,
        CircleUpdateTokenGenRequest,
        CircleAccessSetting,
    ))
)]
pub struct ApiDoc;

// GETヘルスチェック
#[utoipa::path(context_path = "/api")]
#[get("/get-healthcheck")]
pub fn get_healthcheck() -> &'static str {
    "Hello, world!"
}

// POSTヘルスチェック
#[utoipa::path(context_path = "/api")]
#[post("/post-healthcheck", data = "<data>")]
pub fn post_healthcheck(data: Json<HealthCheckRequest>) -> String {
    format!("Accepted post request! {:?}", data.text)
}

// zipダウンロードAPI
#[utoipa::path(context_path = "/api/admin")]
#[post("/download", data = "<request>")]
pub async fn download(
    request: Json<DownloadRequest>, 
    jar: &CookieJar<'_>, 
    app: &State<App>
) -> Result<Json<DownloadResponse>, Status> {
    // バリデーション: パスワードが英数字のみかチェック
    let re = Regex::new(r"^[A-Za-z\d]+$").unwrap();
    if !re.is_match(&request.password) {
        return Err(Status::BadRequest);
    }

    // CookieからJWTの取得
    let jwt = match jar.get("token").map(|c| c.value()) {
        None => return Err(Status::Unauthorized),
        Some(t) => t.to_string(),
    };

    // JWTの検証
    match decode_jwt(&jwt) {
        None => Err(Status::Unauthorized),
        Some(_) => {
            // パスワードの検証
            dotenv().ok();
            let password_hash = env::var("DOWNLOAD_PASSWORD_HASH")
                .expect("download password hash must be set");
            match verify_password_hash(request.password.clone(), password_hash) {
                Ok(_) => {},
                Err(_) => return Err(Status::BadRequest),
            }

            // 各テーブルからデータ取得（失敗時は InternalServerError を返す）
            let students = app.student.get_all().await.map_err(|_| Status::InternalServerError)?;
            let student_pairs = app.student_pair.get_all().await.map_err(|_| Status::InternalServerError)?;
            let assignment_records = app.assignment_record.get_all().await.map_err(|_| Status::InternalServerError)?;
            let lockers = app.locker.get_all().await.map_err(|_| Status::InternalServerError)?;
            let representatives = app.representatives.get_all().await.map_err(|_| Status::InternalServerError)?;
            let organizations = app.organization.get_all().await.map_err(|_| Status::InternalServerError)?;
            let registrations = app.registration.get_all().await.map_err(|_| Status::InternalServerError)?;
            let times = app.time.get_all().await.map_err(|_| Status::InternalServerError)?;

            // CSVファイルの内容をメモリ上で作成するための Vec
            let mut files: Vec<(&str, Vec<u8>)> = Vec::new();

            {
                let mut wtr = Writer::from_writer(vec![]);
                for student in &students {
                    wtr.serialize(student).map_err(|_| Status::InternalServerError)?;
                }
                wtr.flush().map_err(|_| Status::InternalServerError)?;
                let data = wtr.into_inner().map_err(|_| Status::InternalServerError)?;
                files.push(("students.csv", data));
            }
            {
                let mut wtr = Writer::from_writer(vec![]);
                for pair in &student_pairs {
                    wtr.serialize(pair).map_err(|_| Status::InternalServerError)?;
                }
                wtr.flush().map_err(|_| Status::InternalServerError)?;
                let data = wtr.into_inner().map_err(|_| Status::InternalServerError)?;
                files.push(("student_pairs.csv", data));
            }
            {
                let mut wtr = Writer::from_writer(vec![]);
                for record in &assignment_records {
                    wtr.serialize(record).map_err(|_| Status::InternalServerError)?;
                }
                wtr.flush().map_err(|_| Status::InternalServerError)?;
                let data = wtr.into_inner().map_err(|_| Status::InternalServerError)?;
                files.push(("assignment_records.csv", data));
            }
            {
                let mut wtr = Writer::from_writer(vec![]);
                for locker in &lockers {
                    wtr.serialize(locker).map_err(|_| Status::InternalServerError)?;
                }
                wtr.flush().map_err(|_| Status::InternalServerError)?;
                let data = wtr.into_inner().map_err(|_| Status::InternalServerError)?;
                files.push(("lockers.csv", data));
            }
            {
                let mut wtr = Writer::from_writer(vec![]);
                for rep in &representatives {
                    wtr.serialize(rep).map_err(|_| Status::InternalServerError)?;
                }
                wtr.flush().map_err(|_| Status::InternalServerError)?;
                let data = wtr.into_inner().map_err(|_| Status::InternalServerError)?;
                files.push(("representatives.csv", data));
            }
            {
                let mut wtr = Writer::from_writer(vec![]);
                for org in &organizations {
                    wtr.serialize(org).map_err(|_| Status::InternalServerError)?;
                }
                wtr.flush().map_err(|_| Status::InternalServerError)?;
                let data = wtr.into_inner().map_err(|_| Status::InternalServerError)?;
                files.push(("organizations.csv", data));
            }
            {
                let mut wtr = Writer::from_writer(vec![]);
                for reg in &registrations {
                    wtr.serialize(reg).map_err(|_| Status::InternalServerError)?;
                }
                wtr.flush().map_err(|_| Status::InternalServerError)?;
                let data = wtr.into_inner().map_err(|_| Status::InternalServerError)?;
                files.push(("registrations.csv", data));
            }
            {
                let mut wtr = Writer::from_writer(vec![]);
                for time in &times {
                    wtr.serialize(time).map_err(|_| Status::InternalServerError)?;
                }
                wtr.flush().map_err(|_| Status::InternalServerError)?;
                let data = wtr.into_inner().map_err(|_| Status::InternalServerError)?;
                files.push(("times.csv", data));
            }

            // ZIPアーカイブをメモリ上で作成
            let mut zip_buffer = Cursor::new(Vec::new());
            let options: FileOptions<ExtendedFileOptions> = FileOptions::default().compression_method(CompressionMethod::Deflated);
            {
                let mut zip_writer = ZipWriter::new(&mut zip_buffer);
                for (filename, data) in files {
                    zip_writer.start_file(filename, options.clone())
                        .map_err(|_| Status::InternalServerError)?;
                    zip_writer.write_all(&data)
                        .map_err(|_| Status::InternalServerError)?;
                }
                zip_writer.finish().map_err(|_| Status::InternalServerError)?;
            }
            let zip_bytes = zip_buffer.into_inner();

            // 現在時刻を "yymmddhhmmss" 形式で取得
            let timestamp = Utc::now().format("%y%m%d%H%M%S").to_string();

            // ZipResponse 構造体を返す
            Ok(Json(DownloadResponse {
                zip_data: zip_bytes,
                filename: format!("Database_{}.zip", timestamp),
            }))
        }
    }
}
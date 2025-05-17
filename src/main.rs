use tus_yuurikai_system::{infrastructure::router::{App, AppOption}, adapters::controller::ApiDoc};
use tus_yuurikai_system::adapters::controller::{*, locker::*, circle::*};

use rocket::{routes, fs::{FileServer, relative, NamedFile}};
use rocket_cors::{CorsOptions, AllowedOrigins};
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;
use std::{path::{Path, PathBuf}, env};


const FRONTEND_BUILD_PATH: &str = "frontend/build";

#[rocket::get("/<file..>")]
async fn catch_all(file: PathBuf) -> Option<NamedFile> {
    let path = Path::new(FRONTEND_BUILD_PATH).join(&file);

    if path.is_file() {
        // 静的ファイルが存在する場合はそれを返す
        NamedFile::open(path).await.ok()
    } else if path.extension().is_none() {
        // 拡張子がないリクエスト（例: /dashboard, /user/123）には index.html を返す（SPA対応）
        NamedFile::open(Path::new(FRONTEND_BUILD_PATH).join("index.html")).await.ok()
    } else {
        // ファイルでもない・SPAルートでもない場合は 404 を返す
        None
    }
}


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    // コマンドライン引数の受け取り
    let args: Vec<String> = env::args().collect();

    let mut app_option = AppOption::new();

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "same-student" => {
                app_option.same_student_enable = true;
                println!("option same student changed.");
            },
            "local-mail" => {
                app_option.local_mail_enable = true;
                println!("option local mail changed.");
            },
            _ => {
                panic!("Error: Invalid option.")
            }
        }
    }

    // CORSの設定
    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::all(), // すべてのオリジンを許可
        allowed_methods: vec!["GET", "POST", "OPTIONS"]
            .into_iter()
            .map(|s| s.parse().unwrap())
            .collect(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("CORS設定に失敗しました");

    let app = App::new(app_option);
    let _rocket = rocket::build()
        .manage(app)
        .attach(cors)
        .mount(
            "/api",
            routes![
                get_healthcheck,
                post_healthcheck,
                login,
                logout,
            ]
        )
        .mount(
            "/api/admin",
            routes![
                download,
            ]
        )
        .mount(
            "/api/admin/locker",
            routes![
                user_search,
                reset,
            ],
        )
        .mount(
            "/api/admin/circle",
            routes![
                access_setting_post,
                circle_list,
                circle_status_update,
            ],
        )
        .mount(
            "/api/locker",
            routes![
                token_generator,
                main_auth,
                co_auth,
                auth_check,
                locker_register,
                availability
            ],
        )
        .mount(
            "/api/circle",
            routes![
                register_token_generator,
                update_entry,
                update_token_generator,
                circle_main_auth,
                circle_co_auth,
                circle_status,
                access_setting_get,
            ]
        )
        .mount("/", FileServer::from(relative!("frontend/build")))
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .mount("/", routes![catch_all])
        .ignite().await?
        .launch().await?;

    Ok(())
    // Ok(infrastracture::router::run())
}
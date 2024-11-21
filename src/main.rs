use tus_yuurikai_system::{infrastructure::router::App, adapters::controller::ApiDoc};
use tus_yuurikai_system::adapters::controller::{*, locker::*, circle::*};

use rocket::{routes, fs::{FileServer, relative, NamedFile}};
use rocket_cors::{CorsOptions, AllowedOrigins};
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;
use std::path::{Path, PathBuf};

#[rocket::get("/<file..>")]
async fn catch_all(file: PathBuf) -> Option<NamedFile> {
    let path = Path::new("frontend/build").join(file);
    if path.is_file() {
        NamedFile::open(path).await.ok()
    } else {
        NamedFile::open("frontend/build/index.html").await.ok()
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

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

    let app = App::new();
    let _rocket = rocket::build()
        .manage(app)
        .attach(cors)
        .mount(
            "/api",
            routes![
                get_healthcheck,
                post_healthcheck,
                login,
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
            routes![],
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
                update_entry,
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
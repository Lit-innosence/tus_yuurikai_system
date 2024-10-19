use tus_yuurikai_system::{infrastructure::router::App, adapters::controller::ApiDoc};
use tus_yuurikai_system::adapters::controller::{
                                get_healthcheck,
                                post_healthcheck,
                                token_generator,
                                main_auth,
                                co_auth,
                                auth_check,
                                locker_register,
                                user_search,
                                login,
                                availability,
                            };
use rocket::{routes, fs::{FileServer, relative}};
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let app = App::new();
    let _rocket = rocket::build()
        .manage(app)
        .mount("/", FileServer::from(relative!("frontend/build")))
        .mount(
            "/api",
            routes![
                get_healthcheck,
                post_healthcheck,
                login,
            ]
        )
        .mount(
            "/api/admin",
            routes![
                user_search,
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
            routes![]
        )
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .ignite().await?
        .launch().await?;

    Ok(())
    // Ok(infrastracture::router::run())
}
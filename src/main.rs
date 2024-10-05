use tus_yuurikai_system::{infrastracture::router::App, adapters::controller::ApiDoc};
use tus_yuurikai_system::adapters::controller::{
                                get_healthcheck,
                                post_healthcheck,
                                token_generator,
                                main_auth,
                                co_auth,
                                auth_check,
                                locker_register};
use rocket::routes;
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let app = App::new();
    let _rocket = rocket::build()
        .manage(app)
        .mount(
            "/api",
            routes![
                get_healthcheck,
                post_healthcheck,
            ]
        )
        .mount(
            "/api/locker",
            routes![
                token_generator,
                main_auth,
                co_auth,
                auth_check,
                locker_register
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
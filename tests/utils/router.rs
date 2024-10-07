extern crate tus_yuurikai_system;

use tus_yuurikai_system::{infrastracture::router::App, adapters::controller::ApiDoc};
use tus_yuurikai_system::adapters::controller::{
                                get_healthcheck,
                                post_healthcheck,
                                token_generator,
                                main_auth,
                                co_auth,
                                locker_register,
                                user_search
                            };

use rocket::{routes, Rocket, Build};
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;

pub fn rocket() -> Rocket<Build> {
    let app = App::new();
    let rocket = rocket::build()
        .manage(app)
        .mount(
            "/api",
            routes![
                get_healthcheck,
                post_healthcheck
            ]
        )
        .mount(
            "/api/locker",
            routes![
                token_generator,
                main_auth,
                co_auth,
                locker_register,
                user_search
            ],
        )
        .mount(
            "/api/circle",
            routes![
            ],
        )
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
        );

    rocket
}
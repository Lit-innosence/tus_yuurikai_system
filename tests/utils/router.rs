extern crate tus_yuurikai_system;

use tus_yuurikai_system::{infrastructure::router::App, adapters::controller::ApiDoc};
use tus_yuurikai_system::adapters::controller::{
                                get_healthcheck,
                                post_healthcheck,
                                token_generator,
                                main_auth,
                                co_auth,
                                locker_register,
                                user_search,
                                login,
                                availability
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
                post_healthcheck,
                login,
            ]
        )
        .mount(
            "/api/admin",
            routes![
                user_search,
            ]
        )
        .mount(
            "/api/locker",
            routes![
                token_generator,
                main_auth,
                co_auth,
                locker_register,
                availability,
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
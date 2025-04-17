extern crate tus_yuurikai_system;

use tus_yuurikai_system::{infrastructure::router::{App, AppOption}, adapters::controller::ApiDoc};
use tus_yuurikai_system::adapters::controller::{*, locker::*};

use rocket::{routes, Rocket, Build};
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;

pub fn rocket() -> Rocket<Build> {
    let app_option = AppOption::new();
    let app = App::new(app_option);
    rocket::build()
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
            "/api/admin/locker",
            routes![
                user_search,
                reset,
            ]
        )
        .mount(
            "/api/admin/circle",
            routes![
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
        )
}
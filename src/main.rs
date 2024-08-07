use rocket::{Build, Rocket, get, routes};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(paths(
    hello
))]
struct ApiDoc;

#[utoipa::path(context_path = "")]
#[get("/hello")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[rocket::launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![hello])
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>")
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
}


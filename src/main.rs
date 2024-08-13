use rocket::{Build, Rocket, get, post, routes,
            serde::json::Json};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;
use serde::{Serialize, Deserialize};

#[derive(OpenApi)]
#[openapi(
    paths(
        get_healthcheck,
        post_healthcheck,
    ),
    components(schemas(
        HealthCheckRequest,
    ))
)]
struct ApiDoc;

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckRequest {
    #[schema(example = "Hello world from json!")]
    pub text: String
}

// GETヘルスチェック
#[utoipa::path(context_path = "")]
#[get("/get_healthcheck")]
fn get_healthcheck() -> &'static str {
    "Hello, world!"
}

// POSTヘルスチェック
#[utoipa::path(context_path = "")]
#[post("/post_healthcheck", data = "<data>")]
fn post_healthcheck(data: Json<HealthCheckRequest>) -> String {
    format!("Accepted post request! {:?}", data.text)
}

// ユーザー情報登録API

// ロッカー空き状態確認API

// メール認証API

// 完了通知API

// パスワード照合API

// ロッカー利用者検索API

#[rocket::launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![get_healthcheck])
        .mount("/", routes![post_healthcheck])
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>")
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
}


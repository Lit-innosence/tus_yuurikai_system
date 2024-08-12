use rocket::{Build, Rocket, get, post, routes,
            serde::json::Json};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;
use serde::{Serialize, Deserialize};

#[derive(OpenApi)]
#[openapi(
    paths(
        hello,
        test_json,
    ),
    components(schemas(
        TestResult,
    ))
)]
struct ApiDoc;

// jsonテスト用構造体
#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TestResult {
    #[schema(example = "Hello world from json!")]
    pub text: String
}

// ヘルスチェックAPI
#[utoipa::path(context_path = "")]
#[get("/hello")]
fn hello() -> &'static str {
    "Hello, world!"
}

// jsonテスト
#[utoipa::path(context_path = "")]
#[post("/test_json", data = "<data>")]
fn test_json(data: Json<TestResult>) -> String {
    format!("Accepted post request! {:?}", data.text)
}


// ロッカー空き状態確認API

// フォーム内容受取API

// メール認証API

// 完了通知API

// パスワード照合API

// ロッカー利用者検索API

#[rocket::launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![hello])
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>")
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .mount("/", routes![test_json])
}


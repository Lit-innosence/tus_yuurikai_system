use rocket::{Build, Rocket, get, routes};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(paths(
    hello
))]
struct ApiDoc;

// ヘルスチェックAPI
#[utoipa::path(context_path = "")]
#[get("/hello")]
fn hello() -> &'static str {
    "Hello, world!"
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
}


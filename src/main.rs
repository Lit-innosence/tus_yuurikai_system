use rocket::{Build, Rocket, get, routes};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use std::{error::Error, fs::File, io::BufReader, path::Path};
use serde::{Serialize, Deserialize};

#[derive(OpenApi)]
#[openapi(paths(
    hello
))]
struct ApiDoc;

// jsonチェック
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HelloWorld {
    pub text: String
}

// jsonデコード
fn load_json<P: AsRef<Path>>(path: P) -> Result<HelloWorld, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let data = serde_json::from_reader(reader)?;
    Ok(data)
}

// ヘルスチェックAPI
#[utoipa::path(context_path = "")]
#[get("/hello")]
fn hello() -> &'static str {
    "Hello, world!"
}

// jsonテスト
#[utoipa::path(context_path = "")]
#[get("/test_json")]
fn test_json() -> String {
    const FILEPATH: &str= "./src/test.json";

    let data = match load_json(FILEPATH) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error: {}", err);
            return Default::default();
        }
    };
    data.text
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


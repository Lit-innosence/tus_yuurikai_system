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
        user_register,
    ),
    components(schemas(
        HealthCheckRequest,
        UserInfo,
        PairInfo,
        UserRegisterRequest,
    ))
)]
struct ApiDoc;

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckRequest {
    #[schema(example = "Hello world from json!")]
    pub text: String
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserRegisterRequest {
    pub data: PairInfo,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PairInfo {
    pub main_user: UserInfo,
    pub co_user: UserInfo,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    #[schema(example = "4622999")]
    pub student_id: String,
    #[schema(example = "山田")]
    pub family_name: String,
    #[schema(example = "太郎")]
    pub given_name: String,
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
#[utoipa::path(context_path = "")]
#[post("/locker/user-register", data = "<data>")]
fn user_register(data: Json<UserRegisterRequest>) -> String {
    let main_user = &data.data.main_user;
    let co_user = &data.data.co_user;
    format!("Accepted user register request! {:?} {:?}", main_user.student_id, co_user.student_id)
}

// ロッカー空き状態確認API

// メール認証API

// 完了通知API

// パスワード照合API

// ロッカー利用者検索API

#[rocket::launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![get_healthcheck, post_healthcheck, user_register])
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>")
                .url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
}


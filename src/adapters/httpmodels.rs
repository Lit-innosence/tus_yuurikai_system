use crate::domain::{student::UserInfo, student_pair::PairInfo, assignment::AssignmentInfo};

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// ### HealthCheckRequest
///
/// postヘルスチェックのリクエストに使用
#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckRequest {
    #[schema(example = "Hello world from json!")]
    pub text: String,
}

/// ### TokenGenRequest
///
/// token生成、メール送信APIのリクエストに使用
#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TokenGenRequest {
    pub data: PairInfo,
}

/// ### AuthCheckResponse
///
/// 認証検証APIのレスポンスに使用
#[derive(Serialize, ToSchema)]
pub struct AuthCheckResponse {
    pub data: PairInfo,
}

/// ### LockerStatus
///
/// LockerStatusResponseに使用
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LockerStatus{
    pub locker_id: String,
    pub floor: i8,
    pub status: String,
}

/// ### LockerStatusResponse
///
/// ロッカー空き状態確認APIのレスポンスに使用
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct LockerStatusResponse{
    pub data: Vec<LockerStatus>,
}

/// ### LockerResisterRequest
///
/// ロッカー登録APIのリクエストに使用
#[derive(Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LockerResisterRequest {
    pub data: AssignmentInfo,
}

/// ### 管理者パスワード照合APIのリクエストデータ
///
/// username    : ユーザ名
///
/// password    : パスワード
#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginFormRequest{
    #[schema(example = "user000")]
    pub username : String,
    #[schema(example = "0000")]
    pub password : String,
}

/// ### UseSearchResult
///
/// UserSearchResponseに使用する構造体
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserSearchResult {
    pub locker_id : String,
    pub floor : i8,
    pub main_user : UserInfo,
    pub co_user : UserInfo,
    pub year : i32,
}

/// ### UserSearchResponse
///
/// ロッカー利用者検索のレスポンスに使用
#[derive(Debug, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserSearchResponse {
    pub data: Vec<UserSearchResult>,
}
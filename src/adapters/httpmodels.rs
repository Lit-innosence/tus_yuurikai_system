use crate::domain::{assignment::AssignmentInfo, circle::{OrganizationInfo, OrganizationUpdateInfo}, student::UserInfo, student_pair::PairInfo};

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

/// ### LockerTokenGenRequest
///
/// ロッカー予約システムにおいてtoken生成、メール送信APIのリクエストに使用
#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LockerTokenGenRequest {
    pub data: PairInfo,
}

/// ### CircleTokenGenRequest
///
/// 団体登録システムにおいてtoken生成、メール送信APIのリクエストに使用
#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CircleTokenGenRequest {
    pub data: OrganizationInfo,
}

/// ### CircleUpdateTokenGenRequest
///
/// 団体登録システムにおいて情報更新の際のtoken生成、メール送信APIのリクエストに使用
#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CircleUpdateTokenGenRequest {
    pub data: OrganizationUpdateInfo,
}

/// ### AuthCheckResponse
///
/// 認証検証APIのレスポンスに使用
#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthCheckResponse {
    pub data: PairInfo,
    pub auth_id: String,
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
    pub auth_id: String,
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

/// ### LockerResetRequest
///
/// ロッカーリセットのリクエストデータ
#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LockerResetRequest {
    pub password: String,
}

/// ### CircleUpdateRequest
///
/// 団体情報更新のリクエストデータ
#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CircleUpdateRequest {
    pub organization_name: String,
    pub family_name: String,
    pub given_name: String,
    pub student_id: String,
    pub email: String,
}

/// ### OrganizationStatus
///
/// OrganizationStatusResponseに使用する構造体
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationStatus {
    pub organization_id: String,
    pub organization_name: String,
    pub status_acceptance: String,
    pub status_authentication: String,
    pub status_form_confirmation: String,
    pub status_registration_complete: String,
}

/// ### OrganizationStatusResponse
///
/// 団体情報取得に使用
#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationStatusResponse {
    pub data: Vec<OrganizationStatus>,
}

/// ### CircleAccessSetting
///
/// 団体アクセス制限に使用
#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CircleAccessSetting {
    #[schema(example = "2025-01-12T08:00:00.000Z")]
    pub start: String,
    #[schema(example = "2025-01-15T08:00:00.000Z")]
    pub end: String,
}

/// ### OrganizationList
///
/// OrganizationListResponseに使用する構造体
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationList {
    pub organization_id: String,
    pub organization_name: String,
    pub organization_email: String,
    pub main_id: String,
    pub main_family_name: String,
    pub main_given_name: String,
    pub main_email: String,
    pub main_phone: String,
    pub co_id: String,
    pub co_family_name: String,
    pub co_given_name: String,
    pub co_email: String,
    pub co_phone: String,
    pub b_url: String,
    pub c_url: String,
    pub d_url: String,
    pub status_acceptance: String,
    pub status_authentication: String,
    pub status_form_confirmation: String,
    pub status_registration_complete: String,
}

/// ### OrganizationListResponse
///
/// 管理者用団体情報取得に使用
#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationListResponse {
    pub data: Vec<OrganizationList>
}


/// ### OrganizationStatusUpdateRequest
///
/// 団体ステータス更新APIに使用
#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationStatusUpdateRequest {
    pub organization_id: String,
    pub status_acceptance: String,
    pub status_authentication: String,
    pub status_form_confirmation: String,
    pub status_registration_complete: String,
}
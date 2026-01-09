use serde::Serialize;
use sqlx::{prelude::FromRow, types::chrono::NaiveDateTime};

#[derive(Debug, FromRow, Serialize)]
pub struct Ads {
    pub name: String,
    pub is_published: bool,
    pub idx: i32,
    pub start_at: NaiveDateTime,
    pub description: Option<String>,
    pub title: Option<String>,
    pub end_at: NaiveDateTime,
    pub ads_type: String,
    pub priority: i8,
    pub target_country: String,
    pub media_type: Option<String>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct AdsRequest {
    pub user_uuid: String,
    pub user_name: Option<String>,
    pub current_language: Option<String>,
    pub ads_type: String,
    pub user_ip_address: Option<String>,
    pub country: Option<String>,
}

use actix_web::http::StatusCode;
use actix_web::test as actix_test;
use actix_web::{App, body::to_bytes};

use actix_web::ResponseError;
use ads_api::controller::get_country::get_country;
use ads_api::error::APIResponseError;
use ads_api::model::{
    ads::{Ads, AdsRequest},
    queries::{GetAdsQuery, GetClickAdsQuery},
};
use chrono::NaiveDate;
use serde_urlencoded;

#[actix_web::test]
async fn test_api_response_error_status_and_body() {
    let resp = APIResponseError::BadRequest.error_response();
    // check status
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    // check content-type header
    let ct = resp
        .headers()
        .get("content-type")
        .unwrap()
        .to_str()
        .unwrap();
    assert!(ct.starts_with("text/html"));

    // check body
    let body = to_bytes(resp.into_body()).await.unwrap();
    assert_eq!(std::str::from_utf8(&body).unwrap(), "bad request");
}

#[test]
fn test_ads_and_ads_request_serialization() {
    let start = NaiveDate::from_ymd_opt(2020, 1, 1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let end = NaiveDate::from_ymd_opt(2020, 12, 31)
        .unwrap()
        .and_hms_opt(23, 59, 59)
        .unwrap();
    let ads = Ads {
        name: "X".to_string(),
        is_published: true,
        idx: 1,
        start_at: start,
        description: Some("desc".to_string()),
        title: Some("title".to_string()),
        end_at: end,
        ads_type: "Banner".to_string(),
        priority: 1,
        target_country: "Any".to_string(),
        media_type: Some("image".to_string()),
    };

    let s = serde_json::to_string(&ads).unwrap();
    assert!(s.contains("\"name\""));
    assert!(s.contains("\"ads_type\""));

    let req = AdsRequest {
        user_uuid: "u-1".to_string(),
        user_name: Some("Yousef".to_string()),
        current_language: Some("Arabic".to_string()),
        ads_type: "Banner".to_string(),
        user_ip_address: Some("1.2.3.4".to_string()),
        country: Some("Egypt".to_string()),
    };

    let s2 = serde_json::to_string(&req).unwrap();
    assert!(s2.contains("\"user_uuid\""));
    assert!(s2.contains("\"ads_type\""));
}

#[test]
fn test_queries_deserialize() {
    let q = "user_uuid=U1&ads_type=Banner";
    let parsed: GetAdsQuery = serde_urlencoded::from_str(q).unwrap();
    assert_eq!(parsed.user_uuid, "U1");
    assert_eq!(parsed.ads_type, "Banner");

    let q2 = "url=https%3A%2F%2Fexample.com";
    let parsed2: GetClickAdsQuery = serde_urlencoded::from_str(q2).unwrap();
    assert_eq!(parsed2.url, "https://example.com");
}

#[actix_web::test]
async fn test_get_country_no_ip() {
    let req = actix_test::TestRequest::default().to_http_request();
    let res = get_country(&req).await.unwrap();
    assert_eq!(res.0, None);
    assert_eq!(res.1, None);
}

#[actix_web::test]
async fn test_route_configs_register() {
    // ensure ads_cfg and click_ads_cfg can be registered without panic
    let _app = actix_test::init_service(App::new().configure(ads_api::route::ads::ads_cfg)).await;
    let _app2 =
        actix_test::init_service(App::new().configure(ads_api::route::click_ads::click_ads_cfg))
            .await;
}

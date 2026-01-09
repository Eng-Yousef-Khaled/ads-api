use actix_web::{
    HttpResponse, get,
    web::{self, ServiceConfig},
};
use log::error;
use nanoid::nanoid;
use sqlx::{MySql, Pool};

use crate::{error::APIResponseError, model::queries::GetClickAdsQuery};

#[get("/click-ads/{ads}/{uuid}")]
async fn click_ads(
    pool: web::Data<Pool<MySql>>,
    ads: web::Path<(String, String)>,
    detail: web::Query<GetClickAdsQuery>,
) -> Result<HttpResponse, APIResponseError> {
    let uuid = nanoid!(7);
    let _ = sqlx::query(
        "insert
        into `tabSeen Ads`(name,user_uuid, ads, is_clicked, creation, modified, modified_by, owner) VALUES (?,?,?,1, now(), now(),?,?)",
    ).bind(uuid).bind(&ads.1).bind(&ads.0).bind("Administrator")
    .bind("Administrator")
    .execute(pool.get_ref())
    .await
    .map_err(|e| {error!("{}", e);APIResponseError::InternalError})?;
    Ok(HttpResponse::Found()
        .append_header(("Location", detail.url.clone()))
        .finish())
}
pub fn click_ads_cfg(cfg: &mut ServiceConfig) {
    cfg.service(click_ads);
}

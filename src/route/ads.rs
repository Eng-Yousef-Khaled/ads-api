use crate::{
    controller::get_country::get_country,
    error::APIResponseError,
    model::{
        ads::{Ads, AdsRequest},
        queries::GetAdsQuery,
    },
};
use actix_web::{
    HttpRequest, get,
    web::{self, Json, Path, ServiceConfig},
};
use log::error;
use nanoid::nanoid;
use sqlx::{MySql, Pool};

#[get("/app_ads")]
async fn current_ads(
    pool: web::Data<Pool<MySql>>,
    details: web::Query<GetAdsQuery>,
    http: HttpRequest,
) -> Result<Json<Ads>, APIResponseError> {
    let (country_opt, client_ip_opt): (Option<String>, Option<String>) =
        get_country(&http).await.map_err(|e| {
            error!("{}", e);
            APIResponseError::InternalError
        })?;

    let req = AdsRequest {
        ads_type: details.ads_type.clone(),
        user_uuid: details.user_uuid.clone(),
        user_name: details.user_name.clone(),
        current_language: details.current_lang.clone(),
        user_ip_address: client_ip_opt,
        country: country_opt,
    };
    let uuid = nanoid!(7);
    let _ = sqlx::query(
        "insert
        into `tabAds Request`(name, ads_type, user_uuid, user_name, current_language, user_ip_address, country, creation, modified, modified_by, owner) VALUES (?,?, ?, ?, ?, ?, ?, now(), now(),?,?)",
    ).bind(uuid)
    .bind(&req.ads_type)
    .bind(&req.user_uuid)
    .bind(&req.user_name)
    .bind(&req.current_language)
    .bind(&req.user_ip_address)
    .bind(&req.country)
    .bind("Administrator")
    .bind("Administrator")
    .execute(pool.get_ref())
    .await
    .map_err(|e| {error!("{}", e);APIResponseError::InternalError})?;
    let res = sqlx::query_as::<_, Ads>(
        "\
            Select 
                name, 
                is_published, 
                idx, 
                start_at,
                description,
                title,end_at,
                ads_type,
                priority,
                target_country,
                media__type as media_type 
            from `tabAds`
            where 
                ads_type = ?
                and
                    target_language in (?, 'Both')
                and
                    start_at <= now()
                and
                    end_at >= now()
                and
                    is_published = 1
            order by priority
        ",
    )
    .bind(&details.ads_type)
    .bind(&details.current_lang)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        error!("{}", e);
        APIResponseError::InternalError
    })?;

    match res {
        None => Err(APIResponseError::NotFound),
        Some(e) => Ok(Json(e)),
    }
}

#[get("/ad/{name}")]
async fn get_ad(
    pool: web::Data<Pool<MySql>>,
    name: Path<String>,
) -> Result<Json<Ads>, APIResponseError> {
    if name.is_empty() {
        return Err(APIResponseError::BadRequest);
    }
    let res = sqlx::query_as::<_, Ads>("Select name, is_published, idx, start_at,description,title,end_at,ads_type,priority,target_country,media__type as media_type from `tabAds` WHERE name = ?")
        .bind(name.into_inner())
        .fetch_optional(pool.get_ref())
        .await.map_err(|_e| APIResponseError::InternalError)?;
    match res {
        Some(v) => Ok(Json(v)),
        None => Err(APIResponseError::NotFound),
    }
}

pub fn ads_cfg(cfg: &mut ServiceConfig) {
    cfg.service(current_ads);
    cfg.service(get_ad);
}

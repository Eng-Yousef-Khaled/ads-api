// user_uuid=GDFGDFTE&user_name=YOUSEF&current_lang=Arabic&ads_type=Banner

use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetAdsQuery {
    pub user_uuid: String,
    pub user_name: Option<String>,
    pub current_lang: Option<String>,
    pub ads_type: String,
}

#[derive(Deserialize)]
pub struct GetClickAdsQuery {
    pub url: String,
}

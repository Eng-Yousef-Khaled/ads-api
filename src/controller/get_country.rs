use std::error::Error;

use actix_web::HttpRequest;

pub async fn get_country(
    req: &HttpRequest,
) -> Result<(Option<String>, Option<String>), Box<dyn Error>> {
    let conn = req.connection_info();
    let ip = conn.realip_remote_addr();

    match ip {
        Some(value) => {
            let country = reqwest::get(format!("https://ipapi.co/{value}/country_name/"))
                .await?
                .text()
                .await?;
            Ok((Some(country), Some(value.to_string())))
        }

        None => Ok((None, None)),
    }
}

use actix_web::{App, HttpServer, middleware::Logger, web};
use dotenv::dotenv;
use sqlx::MySqlPool;

use crate::route::{ads::ads_cfg, click_ads::click_ads_cfg};
mod controller;
mod error;
mod model;
mod route;

#[rustfmt::skip]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    unsafe {
        std::env::set_var("RUST_LOG", "info");
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    env_logger::init();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to create MySQL pool.");
    println!("Run on http://127.0.0.1:8000");
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new().wrap(logger).service(
            web::scope("/v1")
                .app_data(web::Data::new(pool.clone()))
                .configure(ads_cfg).configure(click_ads_cfg)
        )
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}

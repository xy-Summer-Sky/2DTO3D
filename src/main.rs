use std::env;
use actix_cors::Cors;
use actix_files as fs;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use photosprocess::config::routes::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());

    let app_data = photosprocess::pool::app_state::AppState::new();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        App::new()
            .app_data(web::Data::new(app_data.clone()))
            .wrap(cors)
            .service(fs::Files::new("/static", "F:/CODE/GIThub/2DTO3D/photosprocess/assets").show_files_listing())
            .configure(config_user_routes)
    })
        .bind(if environment == "production" {
            "0.0.0.0:8080"
        } else {
            "127.0.0.1:8080"
        })?
        .run()
        .await
}
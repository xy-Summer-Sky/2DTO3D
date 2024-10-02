//! ![uml](ml.svg)
use std::env;
use actix_cors::Cors;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use svg::node::NodeClone;
use photosprocess::pool::app_state::AppState;
use photosprocess::service::SessionData;

#[actix_rt::main]
async fn main() -> actix_web::Result<()> {
let redis_url = match env::var("ENV") {
    Ok(env) if env == "production" => env::var("REDIS_URL"),
    _ => Ok("redis://127.0.0.1:6379".to_string()),
}.unwrap();
    let redis_store = RedisSessionStore::new(redis_url).await.unwrap();
    let secret_key = Key::generate();
    let app_state = AppState::new().await;
    HttpServer::new(move || {
        App::new()
              .app_data(web::Data::new(app_state.clone()))
             .wrap(Logger::default()) // 添加日志中间件
             .wrap(Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
                .max_age(3600))
            .wrap(SessionMiddleware::builder(redis_store.clone(), secret_key.clone())
                .cookie_name("session_id".to_string())
                .cookie_http_only(true)
                .cookie_secure(true)
                .build()
            )
            .configure(photosprocess::config::routes::config_user_routes)

    })
        .bind("0.0.0.0:8081")?
        .run()
        .await?;
    Ok(())


}




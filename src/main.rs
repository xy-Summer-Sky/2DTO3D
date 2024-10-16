//! ![uml](ml.svg)
use actix_cors::Cors;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use photosprocess::pool::app_state::AppState;
use svg::node::NodeClone;
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;
use photosprocess::ApiDoc;

#[actix_rt::main]
async fn main() -> actix_web::Result<()> {
    let redis_url = "redis://:123456@localhost:6379".to_string();
    let redis_store = RedisSessionStore::new(redis_url).await.unwrap();
    let secret_key = Key::generate();
    let app_state = AppState::new().await;
    let openapi = ApiDoc::openapi();
    HttpServer::new(move || {
        App::new()

            .app_data(web::Data::new(app_state.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600)
                    .supports_credentials()


            )
            .wrap(Logger::default()) // 添加日志中间件
            .wrap(
                SessionMiddleware::builder(redis_store.clone(), secret_key.clone())
                    .cookie_name("session_id".to_string())
                    .cookie_http_only(true)
                    .cookie_secure(true)
                    .build(),
            )
            .configure(photosprocess::config::routes::config_user_routes)
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", openapi.clone()))

    })
    .bind("0.0.0.0:8081")?
    .run()
    .await?;
    Ok(())
}

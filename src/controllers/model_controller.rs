use crate::pool::app_state::AppState;
use crate::service::{FileManager, SessionData};
use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use futures_util::StreamExt;
use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use tokio::io::AsyncWriteExt;

pub type RedisPool = Pool<RedisConnectionManager>;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(new_city);
    cfg.service(upload_image);
}

#[get("/new_city/{city_name}/{user_id}")]
pub async fn new_city(app_state: web::Data<AppState>, path: web::Path<(String, i32)>) -> impl Responder {
    let pool = &app_state.pool;
    let (city_name, user_id) = path.into_inner();
    match FileManager::new_city_and_new_directory(&pool, &user_id, &city_name).await {
        Ok(_) => HttpResponse::Ok().body(format!("New city: {}", city_name)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to create city: {}", e)),
    }
}

#[post("/upload_image")]
pub async fn upload_image(req: HttpRequest, mut payload: Multipart, app_state: web::Data<AppState>) -> impl Responder {
    let mut redis_conn = app_state.redis_pool.get().unwrap();
    let session_id_cookie = req.cookie("session_id");

    if let Some(cookie) = session_id_cookie {
        let session_id = cookie.value().to_string();
        println!("Existing session id from cookie: {}", session_id);

        match SessionData::get_session_data_by_id(&mut redis_conn, &session_id).await {
            Ok(session_data) => {
                println!("Session data: {}", session_data);

                while let Some(item) = payload.next().await {
                    let mut field = item.unwrap();
                    let content_disposition = field.content_disposition().unwrap();
                    let filename = content_disposition.get_filename().unwrap();

                    let filepath = format!("./uploads/{}", sanitize_filename::sanitize(&filename));
                    let mut f = web::block(|| std::fs::File::create(filepath)).await.unwrap();

                    while let Some(chunk) = field.next().await {
                        let data = chunk.unwrap();
                        f = web::block(move || f.write_all(&data).map(|_| f)).await.unwrap();
                    }
                }

                HttpResponse::Ok().body("File uploaded successfully")
            }
            Err(e) => HttpResponse::InternalServerError().body(format!("Failed to get session data: {}", e)),
        }
    } else {
        HttpResponse::BadRequest().body("No session_id cookie found")
    }
}
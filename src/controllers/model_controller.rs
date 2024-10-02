use crate::pool::app_state::AppState;
use crate::service::{FileManager, SessionData};
use actix_web::{get, post, web, HttpResponse, Responder};
use r2d2::Pool;
use std::ops::DerefMut;

use r2d2_redis::RedisConnectionManager;

pub type RedisPool = Pool<RedisConnectionManager>;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(new_city);
    cfg.service(upload_image);
}

#[get("/new_city/{city_name}/{user_id}")]
pub async fn new_city(
    app_state: web::Data<AppState>,
    path: web::Path<(String, i32)>,
) -> impl Responder {
    let pool = &app_state.pool;
    let redis_pool = &app_state.redis_pool;
    let (city_name, user_id) = path.into_inner();
    match FileManager::new_city_and_new_directory(&pool, &user_id, &city_name).await {
        Ok(_) => HttpResponse::Ok().body(format!("New city: {}", city_name)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to create city: {}", e)),
    }
}

#[post("/upload_image")]
pub async fn upload_image(
    image_upload: crate::models::request_models_dto::ImageUpload,
    app_state: web::Data<AppState>,
) -> impl Responder {
    //获取redis连接
    let mut redis_conn_temp = app_state.redis_pool.get().await;
    if let Err(e) = redis_conn_temp {
        return HttpResponse::InternalServerError()
            .body(format!("Failed to get Redis connection: {}", e));
    }
    let mut redis_conn = redis_conn_temp.unwrap().deref_mut();

    //获取session——id
    let session_id = match image_upload.cookie.clone() {
        Some(cookie) => cookie,
        None => return HttpResponse::BadRequest().body("Missing session_id cookie"),
    };
    //利用redis解析session_id,获取session_data,目前不获取任何信息
    let session_data = SessionData::get_session_data_by_id(redis_conn, &session_id).await;

    //保存上传的图片
    let city_id = image_upload.user_info.city_id.to_string();
    let user_id = image_upload.user_info.user_id.to_string();
    let file_content = image_upload.image;


}

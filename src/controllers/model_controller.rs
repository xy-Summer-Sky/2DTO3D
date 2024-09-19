use actix_web::{web, HttpResponse, Responder};
use actix_web::web::Data;
use diesel::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::pool::app_state;
use crate::pool::app_state::{AppState, DbPool};

/// Fetches a list of models.
///
/// # Returns
///
/// * `HttpResponse` - Response containing the list of models.
pub async fn list_models() -> impl Responder {
    HttpResponse::Ok().body("List of models")
}

/// Fetches details of a specific model.
///
/// # Arguments
///
/// * `model_id` - ID of the model to fetch.
///
/// # Returns
///
/// * `HttpResponse` - Response containing the model details.
pub async fn get_model(model_id: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("Details of model {}", model_id))
}
//Extract-contour-api_dbpool
pub async fn extract_contour_api_dbpool(pool:web::Data<DbPool>,model_id: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("Extract contour api dbpool {}", model_id))
}

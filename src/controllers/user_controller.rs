//! User Controller
//!
//! This controllers handles user related APIs.

use actix_web::{web, Responder, HttpResponse};

/// Fetches a list of users.
///
/// # Returns
///
/// * `HttpResponse` - Response containing the list of users.
pub async fn list_users() -> impl Responder {
    HttpResponse::Ok().body("List of users")
}

/// Fetches details of a specific user.
///
/// # Arguments
///
/// * `user_id` - ID of the user to fetch.
///
/// # Returns
///
/// * `HttpResponse` - Response containing the user details.
pub async fn get_user(user_id: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("Details of user {}", user_id))
}
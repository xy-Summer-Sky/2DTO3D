use actix_web::{web, Responder, HttpResponse};
use actix_multipart::Multipart;
use futures_util::stream::StreamExt as _;
use std::io::Write;
use std::fs;
use std::sync::{Arc, Mutex};
use sanitize_filename::sanitize;

/// Handles file upload and saves the file to the specified directory, returning the file URL.
///
/// # Arguments
///
/// * `payload` - Multipart form data containing the file.
///
/// # Returns
///
/// * `HttpResponse` - Response containing the file URL.
pub async fn save_file(mut payload: Multipart) -> impl Responder {
    let mut file_url = String::new();

    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();
        let content_disposition = field.content_disposition().unwrap();
        let filename = content_disposition.get_filename().unwrap().to_string();
        let filepath = format!("./assets/models/custom/upload/{}", sanitize(&filename));

        let file_result = web::block(move || fs::File::create(filepath.clone())).await.unwrap();
        let file = match file_result {
            Ok(file) => Arc::new(Mutex::new(file)),
            Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to create file: {}", e)),
        };

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            let file = Arc::clone(&file);
            let write_result = web::block(move || {
                let mut f = file.lock().unwrap();
                f.write_all(&data).map(|_| ())
            }).await.unwrap();

            if let Err(e) = write_result {
                return HttpResponse::InternalServerError().body(format!("Failed to write to file: {}", e));
            }
        }

        file_url = format!("/static/models/custom/upload/{}", filename);
    }

    HttpResponse::Ok().json(serde_json::json!({ "fileUrl": file_url }))
}
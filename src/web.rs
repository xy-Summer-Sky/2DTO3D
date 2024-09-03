

use crate::filemanager::file_save::svg_save_process::PathType;
use crate::filemanager::save_file;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn save_svg(file_name: web::Path<String>, content: String) -> impl Responder {
    match save_file(&file_name, &content, PathType::ModelBaseSaveSvgFile) {
        Ok(_) => HttpResponse::Ok().body("File saved successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error saving file: {}", e)),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/save_svg/{file_name}").route(web::post().to(save_svg)));
}

pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(init))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

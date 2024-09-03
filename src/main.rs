use actix_cors::Cors;
use actix_files as fs;
use actix_web::{web, App, HttpServer};
use photosprocess::controllers::{model_controller, upload_controller, user_controller};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        App::new()
            .wrap(cors)
            .service(fs::Files::new("/static", "F:/CODE/GIThub/2DTO3D/photosprocess/assets").show_files_listing())
            .route("/upload", web::post().to(upload_controller::save_file))
            .route("/models", web::get().to(model_controller::list_models))
            .route("/models/{id}", web::get().to(model_controller::get_model))
            .route("/users", web::get().to(user_controller::list_users))
            .route("/users/{id}", web::get().to(user_controller::get_user))
           // .route("/generate_model_positions", web::post().to(controllers::model_controller::generate_model_positions_handler))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
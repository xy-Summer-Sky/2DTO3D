pub mod user_controller;
pub mod model_controller;
pub mod upload_controller;


use actix_web::web;

pub fn config_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .configure(crate::controllers::user_controller::config)
    );
}

use actix_web::web;

pub fn config_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .configure(crate::controllers::user_controller::config)
    );

    cfg.service(
        web::scope("/model")
            .configure(crate::controllers::model_controller::config)
    );
}

use actix_web::web;
use utoipa_swagger_ui::SwaggerUi;

pub fn config_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user").configure(crate::controllers::user_controller::config));

    cfg.service(web::scope("/model").configure(crate::controllers::model_controller::config));

    cfg.service(web::scope("/upload").configure(crate::controllers::upload_controller::config));

    // cfg.service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", openapi.clone()));
}

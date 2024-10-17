use utoipa::OpenApi;
use crate::controllers;
#[derive(OpenApi)]
#[openapi(
    paths(
        controllers::model_controller::get_city_ids_by_user_id,
        controllers::model_controller::get_model_by_id,
        controllers::model_controller::get_model_ids,
        controllers::model_controller::new_city,
        controllers::model_controller::upload_image,
        controllers::model_controller::extract_contours,
        controllers::model_controller::build_model,
        controllers::model_controller::get_city_models,

    ),
    components(schemas()),
    tags(
        (name = "ModelController", description = "API for managing models")
    )
)]
pub struct ApiDoc;
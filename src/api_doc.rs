use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(

    ),
    components(schemas()),
    tags(
        (name = "ModelController", description = "API for managing models")
    )
)]
pub struct ApiDoc;
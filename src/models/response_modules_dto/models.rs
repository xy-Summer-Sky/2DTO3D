use serde_derive::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SingleModelResponse {
    pub id: i32,
    pub model_content: String,
}

impl SingleModelResponse {
    pub fn new(id: i32, model_content: String) -> Self {
        SingleModelResponse {
            id,
            model_content,
        }
    }
}
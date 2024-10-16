use crate::dao::ModelModule::ModelDao;
use crate::pool::app_state::DbPool;

pub struct ModelsManagement;
impl ModelsManagement {
    pub async fn get_city_models_by_city_id(
        pool: &DbPool,
        city_id: i32,
    ) -> Result<Vec<(i32, String)>, String> {
        let models = ModelDao::get_models_by_city_id(pool, city_id)
            .map_err(|e| format!("Failed to get city models: {:?}", e))?;
        let mut result = Vec::new();
        for model in models {
            let obj_data = std::fs::read_to_string(&model.model_path)
                .map_err(|e| format!("Failed to read model file: {:?}", e))?;
            result.push((model.id, obj_data));
        }
        Ok(result)
    }

    pub async fn get_model_by_id(pool: &DbPool, model_id: i32) -> Result<(i32, String), String> {
        let model = ModelDao::get_model_by_id(pool, model_id)
            .map_err(|e| format!("Failed to get model by id: {:?}", e))?;
        let obj_data = std::fs::read_to_string(&model.model_path)
            .map_err(|e| format!("Failed to read model file: {:?}", e))?;
        Ok((model.id, obj_data))
    }
    pub async fn get_model_ids_by_city_id(pool: &DbPool, city_id: i32) -> Result<Vec<i32>, String> {
        let models = ModelDao::get_models_by_city_id(pool, city_id)
            .map_err(|e| format!("Failed to get city models: {:?}", e))?;
        let model_ids = models.into_iter().map(|model| model.id).collect();
        Ok(model_ids)
    }
}

use crate::dao::CityDao;
use crate::pool::app_state::DbPool;

pub struct CitiesManagement;

impl CitiesManagement {
    pub fn get_city_ids_by_user_id(pool: &DbPool, user_id: i32) -> Result<Vec<i32>, String> {
        CityDao::get_city_ids_by_user_id(pool, user_id)
            .map_err(|e| format!("Failed to get city ids: {:?}", e))
    }
}

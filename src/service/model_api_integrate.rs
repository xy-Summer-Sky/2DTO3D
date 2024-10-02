use crate::dao::CityDao;

struct ModelApiIntegrate;

impl ModelApiIntegrate {
    async fn new_city(
        city_name: &str,
        city_description: &str,
        pool: &crate::pool::app_state::DbPool,
    ) -> Result<(), String> {
        let new_city = crate::models::entity::city::NewCity {
            city_name: city_name.to_string(),
            user_id: None,
        };
        CityDao::create_city(pool, &new_city)
            .map_err(|e| format!("Failed to insert city: {:?}", e))?;

        Ok(())
    }
}
// src/dao/CityDao.rs
use diesel::prelude::*;
use crate::schema::cities;
use crate::entity::city::City;
use crate::pool::app_state::DbPool;

pub struct CityDao;

impl CityDao {
    pub fn create_city(pool: &DbPool, city: &City) -> QueryResult<usize> {
        let mut conn = pool.get().expect("Failed to get a connection from the pool");
        diesel::insert_into(cities::table)
            .values(city)
            .execute(&mut conn)
    }

    pub fn get_city_by_id(pool: &DbPool, id: i32) -> QueryResult<City> {
        let mut conn = pool.get().expect("Failed to get a connection from the pool");
        cities::table.find(id).first(&mut conn)
    }

    pub fn update_city(pool: &DbPool, city_id: i32, city: &City) -> QueryResult<usize> {
        let mut conn = pool.get().expect("Failed to get a connection from the pool");
        diesel::update(cities::table.find(city_id))
            .set((
                cities::user_id.eq(city.user_id),
                cities::city_name.eq(&city.city_name),
                cities::is_single_model.eq(city.is_single_model),
                cities::model_path.eq(&city.model_path),
            ))
            .execute(&mut conn)
    }

    pub fn delete_city(pool: &DbPool, city_id: i32) -> QueryResult<usize> {
        let mut conn = pool.get().expect("Failed to get a connection from the pool");
        diesel::delete(cities::table.find(city_id)).execute(&mut conn)
    }
}
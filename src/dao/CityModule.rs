// src/dao/CityModule
use crate::models::entity::city::{City, NewCity};
use crate::pool::app_state::DbPool;
use crate::schema::cities;
use diesel::prelude::*;

pub struct CityDao;

impl CityDao {

    //生成新的城市模型记录，只用到了user_id和city_name——不涉及创建对应的模型目录
    //目录构建位于file_manager.rs
    pub fn create_city(pool: &DbPool, city: &NewCity) -> QueryResult<i32> {
        let mut conn = pool
            .get()
            .expect("Failed to get a connection from the pool");
        let transaction_result = conn.transaction::<_, diesel::result::Error, _>(|conn| {
            diesel::insert_into(cities::table)
                .values(city)
                .execute(conn)?;

            let city_id: i32 = diesel::select(diesel::dsl::sql::<diesel::sql_types::Integer>(
                "LAST_INSERT_ID()",
            ))
            .get_result(conn)?;

            Ok(city_id)
        })?;
        Ok(transaction_result)
    }
    pub fn get_city_by_id(pool: &DbPool, id: i32) -> QueryResult<City> {
        let mut conn = pool
            .get()
            .expect("Failed to get a connection from the pool");
        cities::table.find(id).first(&mut conn)
    }

    pub fn update_city(pool: &DbPool, city_id: i32, city: &City) -> QueryResult<usize> {
        let mut conn = pool
            .get()
            .expect("Failed to get a connection from the pool");
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
        let mut conn = pool
            .get()
            .expect("Failed to get a connection from the pool");
        diesel::delete(cities::table.find(city_id)).execute(&mut conn)
    }

    pub fn create_city_with_model_path(
        pool: &DbPool,
        new_city: &NewCity,
    ) -> QueryResult<(i32, String)> {
        let mut conn = pool
            .get()
            .expect("Failed to get a connection from the pool");

        define_sql_function!(fn last_insert_id() -> Unsigned<Integer>);

        let transaction_result = conn.transaction::<_, diesel::result::Error, _>(|conn| {
            // 插入新城市并尝试获取最后插入的ID
            diesel::insert_into(cities::table)
                .values(new_city)
                .execute(conn)?;

            let city_id: u32 = diesel::select(last_insert_id()).get_result(conn)?;

            let model_path: String =
                format!("data/{}/{}", new_city.user_id.unwrap_or_default(), city_id);

            // 在事务中更新model_path
            diesel::update(cities::table.find(city_id as i32))
                .set(cities::model_path.eq(&model_path))
                .execute(conn)?;

            // 返回类型明确指定
            Ok((city_id as i32, model_path)) as QueryResult<(i32, String)>
        });

        transaction_result
    }
}

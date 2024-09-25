use crate::models::entity::model::Model;
use crate::pool::app_state::DbPool;
use crate::schema::models;
use diesel::prelude::*;

pub struct ModelDao;


impl ModelDao {
    pub async fn  create_model(pool: &DbPool, model: &Model) -> QueryResult<usize> {
        let mut conn = pool.get().expect("Failed to get a connection from the pool");
        diesel::insert_into(models::table)
            .values(model)
            .execute(&mut conn)

    }

    pub fn get_model_by_id(pool: &DbPool, model_id: i32) -> QueryResult<Model> {
        let mut conn = pool.get().expect("Failed to get a connection from the pool");
        models::table.find(model_id).first(&mut conn)
    }

    pub fn update_model(pool: &DbPool, model_id: i32, model: &Model) -> QueryResult<usize> {
        let mut conn = pool.get().expect("Failed to get a connection from the pool");
        diesel::update(models::table.find(model_id))
            .set(models::model_path.eq(&model.model_path))
            .execute(&mut conn)
    }

    pub fn delete_model(pool: &DbPool, model_id: i32) -> QueryResult<usize> {
        let mut conn = pool.get().expect("Failed to get a connection from the pool");
        diesel::delete(models::table.find(model_id)).execute(&mut conn)
    }
}
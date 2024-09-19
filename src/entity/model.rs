// src/entity/model.rs
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use crate::schema::models;
#[derive(Insertable,Queryable, Serialize, Deserialize)]
#[diesel(table_name = models)]
pub struct Model {
    pub id: i32,
    pub city_id:Option<i32>,
    pub model_path: String,
}
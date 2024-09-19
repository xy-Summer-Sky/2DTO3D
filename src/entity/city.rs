// src/entity/city.rs
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use crate::schema::cities;
#[derive(Insertable,Queryable, Serialize, Deserialize)]
#[diesel(table_name = cities)]
pub struct City {
    pub id: i32,
    pub user_id: Option<i32>,
    pub city_name: String,
    pub is_single_model: bool,
    pub model_path: Option<String>,
}
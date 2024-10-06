use crate::schema::cities;
// src/entity/city.rs
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = cities)]
pub struct City {
    pub id: i32,
    pub user_id: Option<i32>,
    pub city_name: String,
    pub is_single_model: Option<bool>,
    pub model_path: Option<String>,
    pub svg_height: Option<f32>,
    pub svg_width: Option<f32>,
}

#[derive(Insertable, Queryable)]
#[diesel(table_name = cities)]
pub struct NewCity {
    #[diesel(column_name = user_id)]
    pub user_id: Option<i32>,
    #[diesel(column_name = city_name)]
    pub city_name: String,
}

// src/entity/model.rs
use crate::schema::models;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = models)]
pub struct Model {
    pub id: i32,
    pub city_id: Option<i32>,
    pub model_path: String,
    pub svg_id: Option<i32>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = models)]
pub struct NewModel {
    #[diesel(column_name = city_id)]
    pub city_id: Option<i32>,
    #[diesel(column_name = model_path)]
    pub model_path: String,
    #[diesel(column_name = svg_id)]
    pub svg_id: Option<i32>,
}
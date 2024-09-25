// src/entity/svg.rs
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use crate::schema::svgs;
#[derive(Insertable,Queryable, Serialize, Deserialize)]
#[diesel(table_name = svgs)]
pub struct Svg {
    pub id: i32,
    pub city_id: Option<i32>,
    pub svg_path: String,
    pub image_id: Option<i32>,
    pub image_path:Option<String>
}
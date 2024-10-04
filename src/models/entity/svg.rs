// src/entity/svg.rs
use crate::schema::svgs;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = svgs)]
pub struct Svg {
    pub id: i32,
    pub city_id: Option<i32>,
    pub svg_path: String,
    pub image_id: Option<i32>,
    pub image_path: Option<String>,
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = svgs)]
pub struct NewSvg {
    #[diesel(column_name = city_id)]
    pub city_id: Option<i32>,
    #[diesel(column_name = svg_path)]
    pub svg_path: String,
    #[diesel(column_name = image_id)]
    pub image_id: Option<i32>,
    #[diesel(column_name = image_path)]
    pub image_path: Option<String>,
}

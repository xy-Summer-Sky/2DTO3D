use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::map_info;
#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = map_info)]
pub struct MapInfo {
    pub id: i32,
    pub map_key: Option<i64>,
    pub map_value: Option<i64>,
    pub map_type: Option<i32>,
    pub status: Option<String>,
}


#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = map_info)]
pub struct NewMapInfo {
    #[diesel(column_name = map_key)]
    pub map_key: Option<i64>,
    #[diesel(column_name = map_value)]
    pub map_value: Option<i64>,
    #[diesel(column_name = map_type)]
    pub map_type: Option<i32>,
    #[diesel(column_name = status)]
    pub status: Option<String>,
}
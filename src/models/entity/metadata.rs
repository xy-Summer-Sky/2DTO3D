//src/models/entity/metadata.rs
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::metadata;
#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = metadata)]
pub struct MetaData {
    pub id: u64,
    pub object_id: Option<i32>,
    pub metadata_key: Option<String>,
    pub metadata_value: Option<String>,
    pub city_id:Option<i32>,
    pub user_id:Option<i32>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = metadata)]
pub struct NewMetaData {
    #[diesel(column_name = object_id)]
    pub object_id: Option<i32>,
    #[diesel(column_name = metadata_key)]
    pub metadata_key: Option<String>,
    #[diesel(column_name = metadata_value)]
    pub metadata_value: Option<String>,
    #[diesel(column_name = city_id)]
    pub city_id: Option<i32>,
    #[diesel(column_name = user_id)]
    pub user_id: Option<i32>,
}


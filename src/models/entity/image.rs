use diesel::prelude::*;
use diesel::sql_types::{Integer, Nullable, Varchar};
use crate::schema::images;

#[derive(Queryable, Identifiable)]
#[diesel(table_name = images, primary_key(image_id))]
pub struct Image {
    pub image_id: i32,
    pub image_path: Option<String>,
    pub user_id: Option<i32>,
    pub city_id: Option<i32>,
}
#[derive(Insertable)]
#[diesel(table_name = images)]
pub struct NewImage {
    #[diesel(column_name = image_path)]
    pub image_path: Option<String>,

    #[diesel(column_name = user_id)]
    pub user_id: Option<i32>,

    #[diesel(column_name = city_id)]
    pub city_id: Option<i32>,
}
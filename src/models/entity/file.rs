use crate::schema::files;
use diesel::prelude::*;
use diesel::sql_types::{Bigint, Integer, Nullable, Timestamp, Varchar};
#[derive(Queryable, Insertable, AsChangeset)]
#[diesel(table_name = files)]
pub struct File {
    pub file_id: i32,
    pub user_id: Option<i32>,
    pub path: Option<String>,
    pub file_type: Option<String>,
    pub size: Option<i64>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub permissions: Option<String>,
}

#[derive(Queryable, Insertable, AsChangeset)]
#[diesel(table_name = files)]
pub struct NewFile {
    #[diesel(column_name = user_id)]
    pub user_id: Option<i32>,
    #[diesel(column_name = path)]
    pub path: Option<String>,
    #[diesel(column_name = file_type)]
    pub file_type: Option<String>,
    #[diesel(column_name = size)]
    pub size: Option<i64>,
    #[diesel(column_name = created_at)]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[diesel(column_name = updated_at)]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[diesel(column_name = permissions)]
    pub permissions: Option<String>,
}

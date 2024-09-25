use diesel::prelude::*;
use diesel::sql_types::{Integer, Nullable, Varchar, Bigint, Timestamp};
use crate::schema::files;
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
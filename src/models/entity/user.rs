use crate::schema::users;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: Option<String>,
    pub password_hash: String,
    pub root_directory_path: Option<String>,
    pub storage_quota: Option<i64>,
    pub storage_used: Option<i64>,
    pub access_level: Option<String>,
    pub encryption_key: Option<String>,
    pub last_access: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Queryable)]
#[diesel(table_name = users)]
pub struct NewUser {
    #[diesel(column_name = username)]
    pub username: String,
    #[diesel(column_name = password_hash)]
    pub password_hash: String,
}

#[derive(Queryable)]
#[diesel(table_name = users)]
pub struct UserLogin {
    #[diesel(column_name = id)]
    pub id: i32,
    #[diesel(column_name = username)]
    pub username: String,

    #[diesel(column_name = email)]
    pub email: Option<String>,

    #[diesel(column_name = password_hash)]
    pub password_hash: String,
}

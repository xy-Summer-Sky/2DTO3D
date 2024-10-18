use crate::models::entity::user::{NewUser, User, UserLogin};
use crate::pool::app_state::DbPool;
use crate::schema::users;
use bcrypt;
use diesel::prelude::*;
use std::fs;
use std::path::Path;

pub struct UserDao;

impl UserDao {
    //创建新用户记录，并且返回用户id
    pub fn create_user(pool: &DbPool, user: &NewUser) -> QueryResult<i32> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        println!("create_user");
        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            diesel::insert_into(users::table)
                .values(user)
                .execute(conn)?;
            let user_id: i32 = diesel::select(diesel::dsl::sql::<diesel::sql_types::Integer>(
                "LAST_INSERT_ID()",
            ))
            .get_result(conn)?;
            Ok(user_id)
        })
    }
    pub fn get_user_by_id(pool: &DbPool, user_id: i32) -> QueryResult<User> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        users::table.find(user_id).first(&mut conn)
    }

    pub fn get_user_by_username(pool: &DbPool, username: &str) -> QueryResult<User> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        users::table
            .filter(users::username.eq(username))
            .first(&mut conn)
    }

    pub fn update_user(pool: &DbPool, user_id: i32, user: &User) -> QueryResult<usize> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        diesel::update(users::table.find(user_id))
            .set((
                users::username.eq(&user.username),
                users::email.eq(&user.email),
                users::password_hash.eq(&user.password_hash),
            ))
            .execute(&mut conn)
    }

    pub fn delete_user(pool: &DbPool, user_id: i32) -> QueryResult<usize> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        diesel::delete(users::table.find(user_id)).execute(&mut conn)
    }

    pub fn verify_user(pool: &DbPool, identifier: &str, password: &str) -> bool {
        let mut conn = pool.get().expect("Failed to get DB connection");

        if let Ok(user) = users::table
            .select((
                users::id,
                users::username,
                users::email,
                users::password_hash,
            ))
            .filter(
                users::username
                    .eq(identifier)
                    .or(users::email.eq(identifier)),
            )
            .first::<UserLogin>(&mut conn)
        {
            bcrypt::verify(password, &user.password_hash).unwrap_or(false)
        } else {
            false
        }
    }

    pub fn register(pool: &DbPool, username: &str, password: &str) -> Result<i32, String> {
        if UserDao::get_user_by_username(pool, username).is_ok() {
            return Err("Username already exists".to_string());
        }

        let user_id = UserDao::create_user(
            pool,
            &NewUser {
                username: username.to_string(),
                password_hash: bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap(),
            },
        )
        .expect("Failed to create user");

        Ok(user_id)
    }

    pub fn after_register_create_directory(pool: &DbPool, userid: i32) -> QueryResult<usize> {
        let user_directory = format!("data/{}", userid.to_string());
        let path = Path::new(&user_directory);
        if !path.exists() {
            fs::create_dir_all(path).expect("Failed to create user directory");
        }

        let mut conn = pool.get().expect("Failed to get DB connection");
        diesel::update(users::table.filter(users::id.eq(userid)))
            .set(users::root_directory_path.eq(Some(user_directory)))
            .execute(&mut conn)
    }
}

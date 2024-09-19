use diesel::prelude::*;
use crate::schema::users;
use bcrypt;
use crate::entity::user::{NewUser, User, UserLogin};
use crate::pool::app_state::DbPool;

pub struct UserDao;


impl UserDao {
    pub  fn create_user(pool: &DbPool, user: &NewUser) -> QueryResult<usize> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        println!("create_user");
        diesel::insert_into(users::table)
            .values(user)
            .execute(&mut conn)

    }

    pub  fn get_user_by_id(pool: &DbPool, user_id: i32) -> QueryResult<User> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        users::table.find(user_id).first(&mut conn)
    }

    pub  fn get_user_by_username(pool: &DbPool, username: &str) -> QueryResult<User> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        users::table.filter(users::username.eq(username)).first(&mut conn)
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

    pub  fn delete_user(pool: &DbPool, user_id: i32) -> QueryResult<usize> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        diesel::delete(users::table.find(user_id)).execute(&mut conn)
    }

    pub  fn verify_user(pool: &DbPool, identifier: &str, password: &str) -> bool {

        let mut conn = pool.get().expect("Failed to get DB connection");

        if let Ok(user) = users::table
            .select((users::username, users::email, users::password_hash))
            .filter(users::username.eq(identifier).or(users::email.eq(identifier)))
            .first::<UserLogin>(&mut conn)

        {
            bcrypt::verify(password, &user.password_hash).unwrap_or(false)

        } else {

            false
        }



    }

    pub  fn register(pool: &DbPool, username: &str, password: &str) -> Result<(), String> {
        if UserDao::get_user_by_username(pool, username).is_ok() {
            return Err("Username already exists".to_string());
        }


        if UserDao::create_user(pool, &NewUser {
            username: username.to_string(),
            password_hash: bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap(),
        }).is_err() {
            return Err("Failed to create user".to_string());
        }

        Ok(())
    }
}

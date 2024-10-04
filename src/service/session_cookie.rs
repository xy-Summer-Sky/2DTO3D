use crate::pool::app_state::RedisPool;
use actix_web::{Error, HttpRequest};
use bb8_redis::redis::{aio::MultiplexedConnection, Commands};
use std::ops::DerefMut;
use uuid::Uuid;

pub struct SessionData {
    pub session_id: String,
    user_id: i32,
    username: String,
    email: String,
    access_level: String,
    last_access: String,
}

impl SessionData {
    fn new(
        session_id: String,
        user_id: i32,
        username: String,
        email: String,
        access_level: String,
        last_access: String,
    ) -> SessionData {
        SessionData {
            session_id,
            user_id,
            username,
            email,
            access_level,
            last_access,
        }
    }

    pub(crate) async fn get_session_data_by_id(
        con: &mut MultiplexedConnection,
        session_id: &str,
    ) -> Result<String, redis::RedisError> {
        let session_data: String = redis::cmd("GET").arg(session_id).query_async(con).await?;
        Ok(session_data)
    }

    pub async fn get_session_data(
        req: HttpRequest,
        redis_pool: &RedisPool,
    ) -> Result<String, Error> {
        let session_id_cookie = req.cookie("session_id");
        println!("Attempting to get a Redis connection from the pool...");
        let mut redis_conn_temp = match redis_pool.get().await {
            Ok(conn) => conn,
            Err(e) => {
                println!("Failed to get Redis connection: {}", e);
                return Err(actix_web::error::ErrorInternalServerError("Failed to get Redis connection"));
            }
        };
        println!("Successfully obtained a Redis connection.");
        let redis_conn = redis_conn_temp.deref_mut();
        if let Some(cookie) = session_id_cookie {
            let session_id = cookie.value().to_string();
            println!("Existing session id from cookie: {}", session_id);
            match SessionData::get_session_data_by_id(redis_conn, &session_id).await {
                Ok(_) => Ok(session_id),
                Err(e) => Err(actix_web::error::ErrorInternalServerError(format!(
                    "Failed to get session data: {}",
                    e
                ))),
            }
        } else {
            let new_sid = Uuid::new_v4().to_string();
            println!("New session id set: {}", new_sid);

            Ok(new_sid)
        }
    }

    pub fn extract_cookie_from_request(req: &HttpRequest) -> String {
        let session_id_cookie = req.cookie("session_id");
        if let Some(cookie) = session_id_cookie {
            cookie.value().to_string()
        } else {
            //如果没有则新建一个session，将session_id存入cookie进行返回
            let new_sid = Uuid::new_v4().to_string();
            new_sid
        }
    }
}

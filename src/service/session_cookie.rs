use std::ops::DerefMut;
use actix_web::{Error, HttpRequest, HttpResponse};
use actix_web::cookie::Cookie;
use redis::AsyncCommands;
use uuid::Uuid;
use crate::pool::app_state::RedisPool;
use r2d2::PooledConnection;
use r2d2_redis::{r2d2, RedisConnectionManager};
use redis::Commands;

pub struct SessionData {
    session_id: String,
    user_id: i32,
    username: String,
    email: String,
    access_level: String,
    last_access: String,
}

impl SessionData {
    fn new(session_id: String, user_id: i32, username: String, email: String, access_level: String, last_access: String) -> SessionData {
        SessionData {
            session_id,
            user_id,
            username,
            email,
            access_level,
            last_access,
        }
    }

    pub(crate) async fn get_session_data_by_id(con: &mut PooledConnection<RedisConnectionManager>, session_id: &String, ) -> Result<String, redis::RedisError> {
        let mut redis_conn = con.deref_mut();
        let session_data: String = redis::cmd("GET")
            .arg(session_id)
            .query_async(&mut redis_conn)
            .await?;
        Ok(session_data)
    }

    pub async fn get_session_data(
        req: HttpRequest,
        redis_pool: &RedisPool,
    ) -> Result<HttpResponse, Error> {
        let session_id_cookie = req.cookie("session_id");
        let mut redis_conn = redis_pool.get().unwrap();

        if let Some(cookie) = session_id_cookie {
            let session_id = cookie.value().to_string();
            println!("Existing session id from cookie: {}", session_id);

            match SessionData::get_session_data_by_id(&mut redis_conn, &session_id).await {
                Ok(session_data) => Ok(HttpResponse::Ok().body(format!("Session data: {}", session_data))),
                Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Failed to get session data: {}", e))),
            }
        } else {
            let new_sid = Uuid::new_v4().to_string();
            println!("New session id set: {}", new_sid);

            Ok(HttpResponse::Ok()
                .cookie(
                    Cookie::build("session_id", new_sid)
                        .http_only(true)
                        .finish(),
                )
                .body("New session ID set, check your cookies"))
        }
    }
}
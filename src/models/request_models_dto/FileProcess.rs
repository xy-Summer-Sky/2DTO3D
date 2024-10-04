use actix_web::error::Error;
use actix_web::{web, FromRequest, HttpRequest};
use futures::future::{self, ready, Ready};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct VideoUploadInfo {
    pub user_id: i32,
    pub city_id: i32,
}

pub struct VideoUpload {
    pub video: actix_web::dev::Payload,
    pub cookie: Option<String>,
    pub info: VideoUploadInfo,
}

impl FromRequest for VideoUpload {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
        let info = match web::Query::<VideoUploadInfo>::from_query(req.query_string()) {
            Ok(query) => query.into_inner(),
            Err(_) => VideoUploadInfo {
                user_id: 0,
                city_id: 0,
            },
        };

        let cookie = req.cookie("session_id").map(|c| c.value().to_owned());

        ready(Ok(VideoUpload {
            video: payload.take(),
            cookie,
            info,
        }))
    }
}
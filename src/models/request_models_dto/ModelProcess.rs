use actix_multipart::MultipartError::Payload;
use actix_web::{web, HttpRequest, FromRequest};
use actix_web::error::Error;
use serde::Deserialize;
use futures::future::{self, Ready};


#[derive(Deserialize)]
pub struct UserInfo {
    pub user_id: i32,
    pub city_id: i32,
}

pub struct ImageUpload {
    pub image: web::Payload,
    pub cookie: Option<String>,
    pub user_info: UserInfo,
}

impl FromRequest for ImageUpload {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut actix_web::web::Payload) -> Self::Future {
        let user_info_result = web::Query::<UserInfo>::from_query(req.query_string());

        match user_info_result {
            Ok(query) => {
                let cookie = req.cookie("session_id").map(|c| c.value().to_owned());
                let image_payload = payload.take();

                let image_payload = std::mem::take(payload);
                future::ready(Ok(ImageUpload {
                    image: image_payload,
                    cookie,
                    user_info: query.into_inner(),
                }))
            },
            Err(e) => {
                future::ready(Err(actix_web::error::ErrorBadRequest("Invalid user info parameters")))
            }
        }
    }
}



use actix_web::{HttpRequest, HttpResponse};

use actix_web::error::ErrorBadRequest;
use futures::StreamExt; // 导入 StreamExt 用于流的处理
use serde::Deserialize;

#[derive(Deserialize, Debug,ToSchema)]
pub struct UserInfo {
    pub user_id: i32,
    pub city_id: i32,
}
#[derive(Deserialize, Debug,ToSchema)]
pub struct ImageUpload {
    pub cookie: Option<String>,
    pub user_info: UserInfo,
    pub image_name: String,
}

/// Example request format for `ExtractContourRequestData`:
///
/// ```http
/// POST /extract_contour HTTP/1.1
/// Host: example.com
/// Content-Type: application/json
/// Cookie: session_id=your_session_id
///
/// {
///     "user_id": 123,
///     "city_id": 456,
///     "image_id": 789,
///     "right_clicks": [
///         {
///             "x": 100,
///             "y": 150,
///             "type": "right"
///         },
///         {
///             "x": 200,
///             "y": 250,
///             "type": "left"
///         }
///     ],
///     "image_data": "QWERTYUIOPASDFGHJKLZXCVBNM=="
/// }
/// ```
impl FromRequest for ImageUpload {
    type Error = actix_web::Error;
    type Future = futures::future::Ready<Result<Self, actix_web::Error>>;

    /// Example request format for `ImageUpload`:
    ///
    /// ```http
    /// POST /upload_image HTTP/1.1
    /// Host: example.com
    /// Content-Type: application/json
    /// Cookie: session_id=your_session_id
    ///
    /// {
    ///     "user_id": 123,
    ///     "city_id": 456,
    ///     "image_name": "example_image.png"
    /// }
    /// ```
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // 尝试解析用户信息
        let user_info = web::Query::<UserInfo>::from_query(req.query_string());

        match user_info {
            Ok(query) => {
                let cookie = req.cookie("session_id").map(|c| c.value().to_string());
                let image_name = req.match_info().get("image_name").unwrap_or("default_image_name").to_string();

                // 使用 ready 创建一个立即解析的 Future
                futures::future::ready(Ok(ImageUpload {
                    cookie,
                    user_info: query.into_inner(),
                    image_name,
                }))
            },
            Err(_) => {
                // 直接返回错误
                ready(Err(ErrorBadRequest("Invalid user info parameters")))
            }
        }
    }
}
pub async fn upload_image_process_image(mut payload: web::Payload) -> Vec<u8> {
    let mut image_data = Vec::new();

    while let Some(Ok(chunk)) = payload.next().await {
        image_data.extend(&chunk);
    }

    image_data
}



//提取轮廓的请求结构体
use serde::{Serialize};


impl std::str::FromStr for Click {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}
/// Example request format for `ExtractContourRequestData`:
///





use futures::future::{ready}; // 确保引入了正确的 Futures 工具


#[derive(Deserialize,Serialize,ToSchema)]
pub struct ExtractContourRequestData {
    pub(crate) user_id: i32,
    pub(crate) city_id: i32,
    pub(crate) image_id: i32,
    right_clicks: Vec<Click>,
    pub image_data: String,

}

#[derive(Deserialize,Serialize,ToSchema)]
struct Click {
    x: i32,
    y: i32,
    #[serde(rename = "type")]
    click_type: String,
}
/// Example request format for `ExtractContourRequestData`:
///
/// ```http
/// POST /extract_contour HTTP/1.1
/// Host: example.com
/// Content-Type: application/json
/// Cookie: session_id=your_session_id
//
// {
//     "user_id": 123,
//     "city_id": 456,
//     "image_id": 789,
//     "right_clicks": [
//         {
//             "x": 100,
//             "y": 150,
//             "type": "right"
//         },
//         {
//             "x": 200,
//             "y": 250,
//             "type": "left"
//         }
//     ],
//     "image_data": "QWERTYUIOPASDFGHJKLZXCVBNM=="
// }
/// ```
/// image_data 是图像数据，通常是经过Base64编码的二进制数据，以字符串形式发送。请注意，由于JSON不支持原生的二进制数据，所以图像数据需要编码。


// use futures::future::{ TryFutureExt, BoxFuture};  // 确保引入 BoxFuture
 // Ensure BoxFuture and FutureExt are imported

use actix_web::{web, FromRequest, Error, dev::Payload};
use actix_web::web::Json;
use futures::future::{self, FutureExt, BoxFuture};
use futures_util::future::Ready;
use futures_util::TryFutureExt;
use svg::node::NodeClone;
use std::sync::Arc;


use futures::future::LocalBoxFuture;
use utoipa::ToSchema;

impl FromRequest for ExtractContourRequestData {
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
        let req = req.clone();
        let mut payload = payload.take();

        async move {
            let json_result = web::Json::<ExtractContourRequestData>::from_request(&req,&mut payload).await;
            match json_result {
                Ok(data) => Ok(data.into_inner()),
                Err(e) => Err(e.into()),
            }
        }
            .boxed_local()
    }
}

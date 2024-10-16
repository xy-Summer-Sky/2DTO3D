use actix_web::web::Payload;
use actix_web::{post, web, Error, HttpRequest, HttpResponse};
use crate::pool::app_state::AppState;

#[post("/convert")]
async fn convert_video( app_state: web::Data<AppState>,req: HttpRequest, payload: Payload) -> Result<HttpResponse, Error> {
    // 提取 HeaderMap 并创建 Multipart 实例
    let headers = req.headers();
    let multipart = actix_multipart::Multipart::new(headers, payload);

    // 调用转换函数
    match crate::utils::convert_video(multipart).await {
        Ok(response) => Ok(response),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}
// 配置路由
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(convert_video);
}

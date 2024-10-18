use actix_web::{http::header, test, App};
// 确保导入 BytesMut
use photosprocess::controllers::upload_controller::config;
use std::fs;
use tokio_util::bytes::BytesMut;
use photosprocess::config::routes::config_user_routes;
use photosprocess::pool::app_state::AppState;

#[actix_rt::test]
async fn test_convert_video() {
    let app_state = AppState::new().await;
    let _ = env_logger::builder().is_test(true).try_init();
    let mut app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(app_state.clone()))
            .configure(config_user_routes),
    )
        .await;
    // 读取视频文件为二进制数据
    let video_data = fs::read("tmp/video.webm").expect("Failed to read video file");

    // 创建 multipart/form-data 请求
    let mut multipart_payload = BytesMut::new();
    multipart_payload.extend_from_slice(
        b"--abc123\r\n\
        Content-Disposition: form-data; name=\"file\"; filename=\"video.webm\"\r\n\
        Content-Type: video/webm\r\n\r\n",
    );
    multipart_payload.extend_from_slice(&video_data);
    multipart_payload.extend_from_slice(b"\r\n--abc123--\r\n");

    // 发送 POST 请求
    let req = test::TestRequest::post()
        .uri("/upload/convert")
        .insert_header((
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("multipart/form-data; boundary=abc123"),
        ))
        .set_payload(multipart_payload.freeze()) // 转换为 Bytes
        .to_request();

    let resp = test::call_service(&mut app, req).await;

    // 验证是否返回200 OK
    assert!(
        resp.status().is_success(),
        "Expected 200 OK, got {}, response: {:?}",
        resp.status(),
        resp.response().body()
    );
}

use actix_multipart::Multipart;
use actix_web::{post, web, HttpServer, Responder};
use futures::StreamExt;
use serde::Deserialize;
use std::process::Command;

#[derive(Deserialize)]
struct ContourParams {
    heightA: f64,
    heightB: f64,
    heightC: f64,
    contourPoints: Vec<(f64, f64)>,
}

#[post("/extract_contour")]
async fn extract_contour(mut payload: Multipart) -> impl Responder {
    while let Some(item) = payload.next().await {
        let mut field = item.expect("Error reading field");
        let mut data = Vec::new();
        while let Some(chunk) = field.next().await {
            data.extend_from_slice(&chunk.expect("Error reading chunk"));
        }
        // 将图片数据传递给Python脚本进行轮廓提取
        let output = Command::new("python3")
            .arg("path/to/your/extract_contour_script.py")
            .arg(base64::encode(&data))
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            let contour_points: Vec<(f64, f64)> = serde_json::from_slice(&output.stdout).expect("Error parsing JSON");
            return web::Json(contour_points);
        } else {
            return web::Json(vec![]);
        }
    }
    web::Json(vec![])
}

#[post("/generate_model")]
async fn generate_model(params: web::Json<ContourParams>) -> impl Responder {
    let height_a = params.heightA;
    let height_b = params.heightB;
    let height_c = params.heightC;
    let contour_points = &params.contourPoints;

    // 调用现有的模型生成逻辑
    let output = Command::new("python3")
        .arg("path/to/your/python_script.py")
        .arg(height_a.to_string())
        .arg(height_b.to_string())
        .arg(height_c.to_string())
        .arg(format!("{:?}", contour_points))
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        format!("Model generated with heights: A={}, B={}, C={}", height_a, height_b, height_c)
    } else {
        format!("Failed to generate model: {:?}", output)
    }
}


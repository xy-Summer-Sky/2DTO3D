use actix_web::{Error, HttpResponse};
use futures::stream::StreamExt;
use futures_util::TryStreamExt;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;


pub async fn convert_video(mut payload: actix_multipart::Multipart) -> Result<HttpResponse, Error> {
    // Iterate over the multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        // Assume only one file
        if let Some(content_disposition) = field.content_disposition().cloned() {
            let filename = content_disposition
                .get_filename()
                .unwrap_or("video.webm");
            let filepath = format!("tmp/{}", filename);

            // Save the uploaded file
            let mut file = match tokio::fs::File::create(&filepath).await {
                Ok(f) => f,
                Err(e) => {
                    println!("Failed to create file: {}", e);
                    return Err(actix_web::error::ErrorInternalServerError(format!("Failed to create file: {}", e)));
                }
            };
            while let Some(chunk) = field.next().await {
                let data = match chunk {
                    Ok(d) => d,
                    Err(e) => {
                        println!("Failed to read chunk: {}", e);
                        return Err(actix_web::error::ErrorInternalServerError(format!("Failed to read chunk: {}", e)));
                    }
                };
                if let Err(e) = file.write_all(&data).await {
                    println!("Failed to write to file: {}", e);
                    return Err(actix_web::error::ErrorInternalServerError(format!("Failed to write to file: {}", e)));
                }
            }

            // Run ffmpeg command to convert the video
            let output_filepath = format!("tmp/output_{}.mp4", uuid::Uuid::new_v4());
            let output = match Command::new("ffmpeg")
                .arg("-i")
                .arg(&filepath)
                .arg("-c:v")
                .arg("copy")
                .arg("-c:a")
                .arg("aac")
                .arg("-b:a")
                .arg("128k")
                .arg(&output_filepath)
                .output()
                .await {
                Ok(o) => o,
                Err(e) => {
                    println!("Failed to execute ffmpeg: {}", e);
                    return Err(actix_web::error::ErrorInternalServerError(format!("Failed to execute ffmpeg: {}", e)));
                }
            };

            return if output.status.success() {
                let video_data = match tokio::fs::read(&output_filepath).await {
                    Ok(data) => data,
                    Err(e) => {
                        println!("Failed to read output file: {}", e);
                        return Err(actix_web::error::ErrorInternalServerError(format!("Failed to read output file: {}", e)));
                    }
                };
                Ok(HttpResponse::Ok()
                    .content_type("video/mp4")
                    .body(video_data))
            } else {
                let error_message = String::from_utf8_lossy(&output.stderr);
                println!("Failed to convert video: {}", error_message);
                Err(actix_web::error::ErrorInternalServerError(format!(
                    "Failed to convert video: {}",
                    error_message
                )))
            }
        }
    }

    println!("No file uploaded");
    Err(actix_web::error::ErrorBadRequest("No file uploaded"))
}
//! Model Controller
//!
//! This controllers handles model related APIs.

use actix_web::{web, HttpResponse, Responder};
use std::fs;

/// Fetches a list of models.
///
/// # Returns
///
/// * `HttpResponse` - Response containing the list of models.
pub async fn list_models() -> impl Responder {
    HttpResponse::Ok().body("List of models")
}

/// Fetches details of a specific model.
///
/// # Arguments
///
/// * `model_id` - ID of the model to fetch.
///
/// # Returns
///
/// * `HttpResponse` - Response containing the model details.
pub async fn get_model(model_id: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("Details of model {}", model_id))
}
use crate::core_algorithm::build_city_model_independent::{generate_model_positions, ModelGridInfo};
use futures_util::AsyncWriteExt;
use serde_json::json;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use zip::write::FileOptions;
use zip::ZipWriter;

pub async fn generate_model_positions_handler() -> impl Responder {
    // Read and parse the grid mappings JSON file
    let data = std::fs::read_to_string("output/mappings/grid_mappings.json").expect("Unable to read file");
    let model_grid_infos: Vec<ModelGridInfo> = serde_json::from_str(&data).expect("Unable to parse JSON");

    // Generate model positions
    match generate_model_positions(&model_grid_infos) {
        Ok(model_positions) => {
            // Write the generated model positions to a JSON file
            let output_json_path = "output/3d_models/model_positions.json";
            let output_json_file = std::fs::File::create(output_json_path).expect("Unable to create output file");
            let writer = std::io::BufWriter::new(output_json_file);
            serde_json::to_writer_pretty(writer, &model_positions).expect("Unable to write to output file");

            // Create a zip file to store the JSON and model files--
            let zip_file_path = "output/3d_models/model_data.zip";
            let zip_file = File::create(&zip_file_path).expect("Unable to create zip file");
            let mut zip = ZipWriter::new(zip_file);

            // Add JSON file to the zip
            zip.start_file("model_positions.json", FileOptions::default()).expect("Unable to add JSON file to zip");
            let json_data = fs::read(output_json_path).expect("Unable to read JSON file");
            zip.write_all(&json_data).expect("Unable to write JSON data to zip");

            // Add model files to the zip
            let model_files_dir = Path::new("path/to/model/files");
            for entry in fs::read_dir(model_files_dir).expect("Unable to read model files directory") {
                let entry = entry.expect("Unable to get directory entry");
                let path = entry.path();
                if path.is_file() {
                    let file_name = path.file_name().unwrap().to_str().unwrap();
                    zip.start_file(file_name, FileOptions::default()).expect("Unable to add model file to zip");
                    let file_data = fs::read(&path).expect("Unable to read model file");
                    zip.write_all(&file_data).expect("Unable to write model data to zip");
                }
            }

            zip.finish().expect("Unable to finish zip file");

            // Return the zip file path
            let response = json!({
                "zip_file": zip_file_path,
            });
            HttpResponse::Ok().json(response)
        },
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}
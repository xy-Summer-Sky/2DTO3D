use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufWriter;

#[derive(Deserialize)]
pub struct ModelGridInfo {
    pub rgb: [u8; 3],
    pub model_path: String,
    pub grid: Vec<[f32; 4]>,
}

#[derive(Serialize)]
pub struct ModelPosition {
    pub model_path: String,
    pub position: [f32; 3],
    pub rotation: [f32; 3],
}

pub fn generate_model_positions(model_grid_infos: &Vec<ModelGridInfo>) -> Result<Vec<ModelPosition>, String> {
    let mut model_positions = Vec::new();

    for model_grid_info in model_grid_infos.iter() {
        for grid in &model_grid_info.grid {
            let model_path = model_grid_info.model_path.clone();
            let position = [grid[0], grid[1], 0.0]; // Use grid values for initial position, z is set to 0.0
            let rotation = [0.0, 0.0, grid[3]]; // Use grid[3] for rotation around z-axis

            model_positions.push(ModelPosition {
                model_path,
                position,
                rotation,
            });
        }
    }

    let output_file = File::create("output/3d_models/model_positions.json")
        .map_err(|e| format!("Unable to create output file: {:?}", e))?;
    let writer = BufWriter::new(output_file);
    serde_json::to_writer_pretty(writer, &model_positions)
        .map_err(|e| format!("Unable to write to output file: {:?}", e))?;

    Ok(model_positions)
}
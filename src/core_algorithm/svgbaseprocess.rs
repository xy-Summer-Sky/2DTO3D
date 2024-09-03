// src/core_algorithm/svgbaseprocess.rs

pub mod generate_model_base_svg {
    use serde::Deserialize;
    use serde_json;
    use std::f32::consts::SQRT_2;
    use std::fs::File;
    use std::io::{self, Read};

    #[derive(Deserialize)]
    pub struct ColorModel {
        pub hsv_range: [[u8; 3]; 2],
        pub model_path: String,
    }

    #[derive(Deserialize)]
    pub struct ColorModelMapping {
        pub colors: std::collections::HashMap<String, ColorModel>,
    }

    pub struct ModelBase {
        width: u32,
        height: u32,
    }

    impl ModelBase {
        pub fn new(width: u32, height: u32) -> Self {
            ModelBase { width, height }
        }

        fn calculate_rotatable_square_size(&self, obj_file_path: &str) -> u32 {
            // Load the OBJ file
            let (models, _materials) = tobj::load_obj(obj_file_path, &tobj::LoadOptions::default())
                .expect("Failed to load OBJ file");

            // Calculate the bounding box of the model
            let mut min_x = f32::MAX;
            let mut min_y = f32::MAX;
            let mut min_z = f32::MAX;
            let mut max_x = f32::MIN;
            let mut max_y = f32::MIN;
            let mut max_z = f32::MIN;

            for model in models {
                for vertex in model.mesh.positions.chunks(3) {
                    let x = vertex[0];
                    let y = vertex[1];
                    let z = vertex[2];

                    if x < min_x {
                        min_x = x;
                    }
                    if y < min_y {
                        min_y = y;
                    }
                    if z < min_z {
                        min_z = z;
                    }
                    if x > max_x {
                        max_x = x;
                    }
                    if y > max_y {
                        max_y = y;
                    }
                    if z > max_z {
                        max_z = z;
                    }
                }
            }

            // Calculate the dimensions of the bounding box
            let width = max_x - min_x;
            let height = max_y - min_y;
            let depth = max_z - min_z;

            // Use the largest dimension to calculate the square side
            let max_dimension = width.max(height);
            // .max(depth);
            let square_side = (max_dimension * SQRT_2).ceil() as u32;
            square_side
        }
        fn generate_svg_base(&self, obj_file_path: &str) -> String {
            let side = self.calculate_rotatable_square_size(obj_file_path);
            let side_in_mm = side * 10;
            format!(
                r#"<svg width="{0}mm" height="{0}mm" xmlns="http://www.w3.org/2000/svg">
            <rect x="0" y="0" width="{0}" height="{0}" fill="none" stroke="black"/>
           </svg>"#,
                side_in_mm
            )
        }

        pub fn generate_svg_base_all(
            &self,
            json_file_path: &str,
        ) -> io::Result<Vec<(String, String)>> {
            // Load the JSON configuration file into a ColorModelMapping
            let mut file = File::open(json_file_path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let mapping: ColorModelMapping = serde_json::from_str(&contents)?;

            let mut svg_files = Vec::new();
            for (color, model) in &mapping.colors {
                let base_svg = self.generate_svg_base(&model.model_path);
                let svg_content = format!(
                    r#"{}
            <g transform="translate(0,0)">
                <use xlink:href="{}" fill="{}"/>
            </g>
        </svg>"#,
                    base_svg.trim_end_matches("</svg>"), // Remove the closing tag to append new content
                    model.model_path,
                    color
                );
                let svg_content_with_namespace = format!(
                    r#"<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">{}</svg>"#,
                    svg_content.trim_start_matches("<svg ")
                );
                let file_name = format!("{}_{}.svg", "generated_model_base", color);
                svg_files.push((file_name, svg_content_with_namespace));
            }
            Ok(svg_files)
        }
    }
}

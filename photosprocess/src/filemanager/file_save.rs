pub mod svg_save_process {
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;
    use crate::core::svgbaseprocess::generate_model_base_svg::ModelBase;

    pub enum PathType {
        ModelBaseSaveSvgFile,
        ModelSave,
        MappingsJson,
        ThreeDModelResults,
        PictureSvgContour,
        MtlFile,
        ObjFile,
    }

    pub fn save_file(file_name: &str, content: &str, path_type: PathType) -> std::io::Result<()> {
        match path_type {
            PathType::ModelBaseSaveSvgFile => save_model_base_svg_file(file_name, content),
            PathType::ModelSave => save_model_file(file_name, content),
            PathType::MappingsJson => save_mappings_json(file_name, content),
            PathType::ThreeDModelResults => save_3d_model_results(file_name, content),
            PathType::PictureSvgContour => save_picture_svg_contour(file_name, content),
            PathType::MtlFile => save_mtl_file(file_name, content),
            PathType::ObjFile => save_obj_file(file_name, content),
        }
    }

    fn save_model_base_svg_file(file_name: &str, svg_content: &str) -> std::io::Result<()> {
        let parts: Vec<&str> = svg_content.split("</svg>").collect();
        for (i, part) in parts.iter().enumerate() {
            if part.trim().is_empty() {
                continue; // Skip empty parts
            }
            let part_file_name = format!("{}_{}.svg", file_name, i + 1);
            let content = format!("{}{}", part, "</svg>");
            save_to_path("output/model_base_svg/", &part_file_name, &content)?;
        }
        Ok(())
    }

    fn save_model_file(file_name: &str, content: &str) -> std::io::Result<()> {
        save_to_path("assets/models/custom/", file_name, content)
    }

    fn save_mappings_json(file_name: &str, content: &str) -> std::io::Result<()> {
        save_to_path("output/mappings/", file_name, content)
    }

    fn save_3d_model_results(file_name: &str, content: &str) -> std::io::Result<()> {
        save_to_path("output/3d_models/", file_name, content)
    }

    fn save_picture_svg_contour(file_name: &str, content: &str) -> std::io::Result<()> {
        save_to_path("output/contour_svgs/", file_name, content)
    }

    fn save_mtl_file(file_name: &str, content: &str) -> std::io::Result<()> {
        save_to_path("output/mtl_files/", file_name, content)
    }

    fn save_obj_file(file_name: &str, content: &str) -> std::io::Result<()> {
        save_to_path("output/obj_files/", file_name, content)
    }

    fn save_to_path(base_path: &str, file_name: &str, content: &str) -> std::io::Result<()> {
        let path = Path::new(base_path).join(file_name);
        let mut file = File::create(&path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }


}
// src/lib.rs
mod colormodel;
mod hsv_processing;
mod svg_generator;
mod transformation;
mod hex_utils;
mod image_processing;
mod photoprocesstest;
mod city_model;
mod web;
mod controller;

pub use colormodel::ColorModel;
pub use colormodel::ColorModelMapping;
pub use hsv_processing::HSVProcessing;
pub use svg_generator::SVGGenerator;
pub use transformation::Transformation;
pub use hex_utils::HexUtils;

pub use city_model::combine_3d_models;
pub use city_model::read_mappings;

pub mod core_algorithm {
    pub mod svgbaseprocess;
    pub use svgbaseprocess::generate_model_base_svg;
}


pub mod file_path{
    pub mod file_path_process;
    pub use file_path_process::file_path_convert;

}

pub mod filemanager{
    pub mod file_save;
    pub use file_save::svg_save_process::save_file;
}
// src/lib.rs
mod city_model;
mod colormodel;
pub mod controllers;
mod hex_utils;
mod hsv_processing;
mod image_processing;
mod photoprocesstest;
mod svg_generator;
mod transformation;
mod web;
mod entity;
mod dbmanager;
mod dao;
mod service;
mod core_algorithm;

pub use colormodel::ColorModel;
pub use colormodel::ColorModelMapping;
pub use hex_utils::HexUtils;
pub use hsv_processing::HSVProcessing;
pub use svg_generator::SVGGenerator;
pub use transformation::Transformation;

pub use city_model::combine_3d_models;
pub use city_model::read_mappings;

pub mod file_path {
    pub mod file_path_process;
    pub use file_path_process::file_path_convert;
}

pub mod filemanager {
    pub mod file_save;
    pub use file_save::svg_save_process::save_file;
}


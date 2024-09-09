// src/lib.rs
pub mod controllers;

mod web;
mod entity;
mod dbmanager;
mod dao;
mod service;
mod core_algorithm;
pub mod api1;

pub mod file_path {
    pub mod file_path_process;
    pub use file_path_process::file_path_convert;
}

pub mod filemanager {
    pub mod file_save;
    pub use file_save::svg_save_process::save_file;
}


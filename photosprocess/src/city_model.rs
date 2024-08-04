// src/city_model.rs
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use std::path::Path;
use roxmltree::Document;
use serde_json::de::Read;

#[derive(Deserialize)]
pub struct Mapping {
    pub color: String,
    pub model_path: String,
    pub svg_image_path: String,
    pub png_image_path: String,
}

pub fn read_mappings(file_path: &str) -> Vec<Mapping> {
    let file = File::open(file_path).expect("Failed to open mappings file");
    let reader = BufReader::new(file);
    from_reader(reader).expect("Failed to parse mappings")
}

pub fn combine_3d_models(mappings: Vec<Mapping>, output_path: &str) {
    let mut vertex_offset = 0;
    let mut combined_obj_data = String::new();

    for mapping in mappings {
        let obj_data = std::fs::read_to_string(&mapping.model_path)
            .expect(&format!("Failed to read OBJ file: {}", &mapping.model_path));

        // 读取并处理SVG图形
        let svg_data = std::fs::read_to_string(&mapping.svg_image_path)
            .expect(&format!("Failed to read SVG file: {}", &mapping.svg_image_path));
        let svg_path = get_svg_path(&svg_data);
        let svg_area = calculate_polygon_area(&svg_path);

        let obj_base_area = calculate_obj_base_area(&obj_data);

        let num_models = (svg_area / obj_base_area).floor() as usize;

        for i in 0..num_models {
            let (svg_x, svg_y) = get_position_in_svg(&svg_path, i, num_models);

            for line in obj_data.lines() {
                if line.starts_with("v ") {
                    let mut coords: Vec<f32> = line[2..]
                        .split_whitespace()
                        .map(|v| v.parse().unwrap())
                        .collect();
                    // 修改模型坐标以匹配SVG路径点的位置
                    coords[0] += svg_x;
                    coords[1] += svg_y;
                    combined_obj_data.push_str(&format!("v {} {} {}\n", coords[0], coords[1], coords[2]));
                } else if line.starts_with("f ") {
                    let face_data: Vec<String> = line.split_whitespace()
                        .skip(1)
                        .map(|v| {
                            let mut indices: Vec<&str> = v.split('/').collect();
                            let binding = (indices[0].parse::<usize>().unwrap() + vertex_offset).to_string();
                            indices[0] = &binding;
                            indices.join("/")
                        })
                        .collect();
                    combined_obj_data.push_str(&format!("f {}\n", face_data.join(" ")));
                }
            }

            vertex_offset += obj_data.lines().filter(|line| line.starts_with("v ")).count();
        }
    }

    let mut output_file = BufWriter::new(File::create(output_path).expect("Failed to create output OBJ file"));
    output_file.write_all(combined_obj_data.as_bytes()).expect("Failed to write combined OBJ data");
}

fn get_svg_path(svg_data: &str) -> Vec<(f32, f32)> {
    let doc = Document::parse(svg_data).expect("Failed to parse SVG data");
    let mut path_points = Vec::new();

    for node in doc.descendants() {
        if node.is_element() && node.has_tag_name("path") {
            if let Some(d_attr) = node.attribute("d") {
                path_points.extend(parse_svg_path(d_attr));
            }
        }
    }

    path_points
}

fn calculate_polygon_area(points: &[(f32, f32)]) -> f32 {
    let mut area = 0.0;
    let n = points.len();

    for i in 0..n {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i + 1) % n];
        area += x1 * y2 - x2 * y1;
    }

    area.abs() / 2.0
}

fn calculate_obj_base_area(obj_data: &str) -> f32 {
    let mut base_points = Vec::new();

    for line in obj_data.lines() {
        if line.starts_with("v ") {
            let coords: Vec<f32> = line[2..]
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect();
            base_points.push((coords[0], coords[1]));
        }
    }

    calculate_polygon_area(&base_points)
}

fn get_position_in_svg(path: &[(f32, f32)], index: usize, total: usize) -> (f32, f32) {
    let n = path.len();
    let step = n / total;
    let (x, y) = path[(index * step) % n];
    (x, y)
}
pub fn calculate_svg_center(svg_data: &str) -> (f32, f32) {
    let doc = Document::parse(svg_data).expect("Failed to parse SVG data");
    let mut path_points = Vec::new();

    for node in doc.descendants() {
        if node.is_element() && node.has_tag_name("path") {
            if let Some(d_attr) = node.attribute("d") {
                path_points.extend(parse_svg_path(d_attr));
            }
        }
    }

    let (sum_x, sum_y) = path_points.iter().fold((0.0, 0.0), |(acc_x, acc_y), &(x, y)| (acc_x + x, acc_y + y));
    let count = path_points.len() as f32;
    (sum_x / count, sum_y / count)
}
fn parse_svg_path(d: &str) -> Vec<(f32, f32)> {
    let mut points = Vec::new();
    let mut current_point = (0.0, 0.0);
    let mut chars = d.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            'M' | 'L' => {
                let x = parse_coordinate(&mut chars);
                let y = parse_coordinate(&mut chars);
                current_point = (x, y);
                points.push(current_point);
            }
            'm' | 'l' => {
                let x = parse_coordinate(&mut chars) + current_point.0;
                let y = parse_coordinate(&mut chars) + current_point.1;
                current_point = (x, y);
                points.push(current_point);
            }
            _ => {}
        }
    }

    points
}

use std::iter::Peekable;

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn parse_coordinate<I>(chars: &mut std::iter::Peekable<I>) -> f32
where
    I: Iterator<Item = char>,
{
    let mut num_str = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_digit(10) || c == '.' || c == '-' {
            num_str.push(c);
            chars.next();
        } else {
            break;
        }
    }
    num_str.parse().unwrap_or(0.0)
}
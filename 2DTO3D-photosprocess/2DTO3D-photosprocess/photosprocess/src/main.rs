// use std::fs;
// use std::path::PathBuf;
// use hsv_processing::HSVProcessing;
// use svg_generator::SVGGenerator;
// use colormodel::ColorModelMapping;
// use photosprocess::{ColorModelMapping, HSVProcessing, SVGGenerator};
//
// fn main() {
//     let hsv_processor = HSVProcessing::new();
//     let svg_generator = SVGGenerator::new();
//
//     // 假设有一个示例图像路径
//     let image_path = PathBuf::from("assets/images/base_texture.jpg");
//     let contours = hsv_processor.process_image(&image_path);
//
//     // 输出目录路径
//     let output_dir = PathBuf::from("output/svgs");
//     let svg_files = svg_generator.generate_svg(contours, &output_dir);
//
//     // 加载颜色模型映射
//     let color_model_mapping = ColorModelMapping::from_file("config/color_model_mapping.json").expect("Failed to load color model mapping");
//
//     // 将每个SVG文件与对应的模型路径映射起来
//     for svg_file in svg_files {
//         let file_name = svg_file.file_name().unwrap().to_str().unwrap();
//         if let Some(color_model) = color_model_mapping.colors.get(file_name) {
//             println!("SVG file: {} is mapped to color model: {:?}", file_name, color_model);
//         } else {
//             println!("No color model mapping found for SVG file: {}", file_name);
//         }
//     }
// }


use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;

use image::{DynamicImage, GenericImageView, GrayImage, Luma, Rgb};
use imageproc::edges::canny;
use roxmltree::Document;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

mod city_model;

#[derive(Deserialize)]
struct ColorModel {
    hsv_range: [[u8; 3]; 2],
    model_path: String,
}

#[derive(Deserialize)]
struct ColorModelMapping {
    colors: HashMap<String, ColorModel>,
}

#[derive(Serialize)]
pub struct Mapping {
    color: String,
    model_path: String,
    svg_image_path: String,
    png_image_path: String,
}

fn convert_rgb_to_hsv(rgb: &Rgb<u8>) -> [u8; 3] {
    let r = rgb[0] as f32 / 255.0;
    let g = rgb[1] as f32 / 255.0;
    let b = rgb[2] as f32 / 255.0;

    let max = r.max(g.max(b));
    let min = r.min(g.min(b));
    let delta = max - min;

    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    let s = if max == 0.0 { 0.0 } else { delta / max };
    let v = max;

    [
        (h / 360.0 * 255.0) as u8,
        (s * 255.0) as u8,
        (v * 255.0) as u8,
    ]
}

// fn extract_hsv_regions(image: &DynamicImage, color_model: &ColorModel) -> Image<Luma<u8>> {
//     let hsv_image = image.to_rgb8();
//     let (width, height) = hsv_image.dimensions();
//     let mut mask = GrayImage::new(width, height);
//
//     for (x, y, pixel) in hsv_image.enumerate_pixels() {
//         let hsv = convert_rgb_to_hsv(pixel);
//         if hsv >= color_model.hsv_range[0] && hsv <= color_model.hsv_range[1] {
//             mask.put_pixel(x, y, Luma([255]));
//         } else {
//             mask.put_pixel(x, y, Luma([0]));
//         }
//     }
//
//     canny(&mask, 50.0, 100.0)
// }

// fn generate_svg(contours: Vec<Vec<(i32, i32)>>, output_dir: &PathBuf, color: &str) -> String {
//     let file_name = format!("{}_contour.svg", color);
//     let file_path = output_dir.join(&file_name);
//     let mut file = File::create(&file_path).expect("Unable to create file");
//     let mut svg_data = String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" style=\"background-color:white;\">");
//     svg_data.push_str("<path d=\"M ");
//     for contour in contours {
//         for (x, y) in contour {
//             svg_data.push_str(&format!("{},{} ", x, y));
//         }
//     }
//     svg_data.push_str("\" fill=\"none\" stroke=\"black\"/></svg>");
//     file.write_all(svg_data.as_bytes()).expect("Unable to write data");
//     file_name
// }

fn extract_hsv_regions(image: &DynamicImage, color_model: &ColorModel) -> Vec<Vec<(i32, i32)>> {
    let hsv_image = image.to_rgb8();
    let (width, height) = hsv_image.dimensions();
    let mut mask = GrayImage::new(width, height);

    for (x, y, pixel) in hsv_image.enumerate_pixels() {
        let hsv = convert_rgb_to_hsv(pixel);
        if hsv[0] >= color_model.hsv_range[0][0] && hsv[0] <= color_model.hsv_range[1][0] &&
            hsv[1] >= color_model.hsv_range[0][1] && hsv[1] <= color_model.hsv_range[1][1] &&
            hsv[2] >= color_model.hsv_range[0][2] && hsv[2] <= color_model.hsv_range[1][2] {
            mask.put_pixel(x, y, Luma([255]));
        } else {
            mask.put_pixel(x, y, Luma([0]));
        }
    }

    let edges = canny(&mask, 50.0, 100.0);
    let contours = find_contours(&edges);
    contours
}

fn find_contours(edges: &GrayImage) -> Vec<Vec<(i32, i32)>> {
    let mut contours = Vec::new();
    let mut visited = vec![vec![false; edges.height() as usize]; edges.width() as usize];

    for y in 0..edges.height() {
        for x in 0..edges.width() {
            if edges.get_pixel(x, y)[0] == 255 && !visited[x as usize][y as usize] {
                let mut contour = Vec::new();
                trace_contour(x as i32, y as i32, edges, &mut visited, &mut contour);
                if !contour.is_empty() {
                    contours.push(contour);
                }
            }
        }
    }
    contours
}

fn trace_contour(x: i32, y: i32, edges: &GrayImage, visited: &mut Vec<Vec<bool>>, contour: &mut Vec<(i32, i32)>) {
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut stack = vec![(x, y)];

    while let Some((cx, cy)) = stack.pop() {
        if cx >= 0 && cy >= 0 && cx < edges.width() as i32 && cy < edges.height() as i32 {
            if edges.get_pixel(cx as u32, cy as u32)[0] == 255 && !visited[cx as usize][cy as usize] {
                visited[cx as usize][cy as usize] = true;
                contour.push((cx, cy));
                for &(dx, dy) in &directions {
                    stack.push((cx + dx, cy + dy));
                }
            }
        }
    }
}

fn generate_svg(contours: Vec<Vec<(i32, i32)>>, output_dir: &PathBuf, color: &str) -> String {
    let file_name = format!("{}_contour.svg", color);
    let file_path = output_dir.join(&file_name);
    let mut file = File::create(&file_path).expect("Unable to create file");
    let mut svg_data = String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" style=\"background-color:white;\">");
    for contour in contours {
        svg_data.push_str("<path d=\"M ");
        for (x, y) in contour.iter() {
            svg_data.push_str(&format!("{},{} ", x, y));
        }
        svg_data.push_str("\" fill=\"none\" stroke=\"black\"/>");
    }
    svg_data.push_str("</svg>");
    file.write_all(svg_data.as_bytes()).expect("Unable to write data");
    file_name
}
// fn calculate_svg_center(svg_data: &str) -> (f32, f32) {
//     let doc = Document::parse(svg_data).expect("Failed to parse SVG data");
//     let mut path_points = Vec::new();
//
//     for node in doc.descendants() {
//         if node.is_element() && node.has_tag_name("path") {
//             if let Some(d_attr) = node.attribute("d") {
//                 path_points.extend(parse_svg_path(d_attr));
//             }
//         }
//     }
//
//     let (sum_x, sum_y) = path_points.iter().fold((0.0, 0.0), |(acc_x, acc_y), &(x, y)| (acc_x + x, acc_y + y));
//     let count = path_points.len() as f32;
//     (sum_x / count, sum_y / count)
// }
fn calculate_svg_center(svg_data: &str) -> (f32, f32) {
    let doc = Document::parse(svg_data).expect("Failed to parse SVG data");
    let mut x_sum = 0.0;
    let mut y_sum = 0.0;
    let mut count = 0.0;

    for node in doc.descendants() {
        if node.is_element() && node.has_tag_name("path") {
            if let Some(d_attr) = node.attribute("d") {
                // 假设d_attr包含简单的'M x,y ...'形式
                let coords: Vec<&str> = d_attr.split_whitespace().collect();
                for i in 1..coords.len() {
                    let pair: Vec<&str> = coords[i].split(',').collect();
                    x_sum += pair[0].parse::<f32>().unwrap();
                    y_sum += pair[1].parse::<f32>().unwrap();
                    count += 1.0;
                }
            }
        }
    }
    if count > 0.0 {
        (x_sum / count, y_sum / count)
    } else {
        (0.0, 0.0) // 默认中心点
    }
}
// 计算SVG尺寸的函数
fn calculate_svg_dimensions(svg_data: &str) -> (f32, f32) {
    let doc = Document::parse(svg_data).expect("Failed to parse SVG data");
    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;

    for node in doc.descendants() {
        if node.is_element() && node.has_tag_name("path") {
            if let Some(d_attr) = node.attribute("d") {
                let coords: Vec<&str> = d_attr.split_whitespace().collect();
                for coord in coords.iter().filter_map(|s| s.split(',').next()) {
                    let x = coord.parse::<f32>().unwrap();
                    if x < min_x { min_x = x; }
                    if x > max_x { max_x = x; }
                }
                for coord in coords.iter().filter_map(|s| s.split(',').last()) {
                    let y = coord.parse::<f32>().unwrap();
                    if y < min_y { min_y = y; }
                    if y > max_y { max_y = y; }
                }
            }
        }
    }

    if min_x == f32::MAX || max_x == f32::MIN || min_y == f32::MAX || max_y == f32::MIN {
        (0.0, 0.0)
    } else {
        ((max_x - min_x), (max_y - min_y))
    }
}

// 根据模型和SVG尺寸计算缩放因子
fn calculate_scale_factor(model_size: (f32, f32), svg_size: (f32, f32)) -> f32 {
    let (model_width, model_height) = model_size;
    let (svg_width, svg_height) = svg_size;

    let scale_x = svg_width / model_width;
    let scale_y = svg_height / model_height;

    scale_x.min(scale_y)  // 使用较小的缩放比例以确保模型完全适应SVG区域
}

fn layout_models_in_svg(model_count: usize, svg_size: (f32, f32), model_size: (f32, f32)) -> Vec<(f32, f32)> {
    let mut positions = Vec::new();
    let (svg_width, svg_height) = svg_size;
    let (model_width, model_height) = model_size;

    // 计算一行可以放多少模型
    let models_per_row = (svg_width / model_width).floor() as usize;

    for i in 0..model_count {
        let x = (i % models_per_row) as f32 * model_width;
        let y = (i / models_per_row) as f32 * model_height;
        if y < svg_height {
            positions.push((x, y));
        }
    }

    positions
}


fn main() {
    // 加载颜色模型映射
    let file = File::open("config/color_model_mapping.json").expect("Failed to open color model mapping file");
    let reader = BufReader::new(file);
    let color_model_mapping: ColorModelMapping = serde_json::from_reader(reader).expect("Failed to parse color model mapping");

    // 读取图片
    let image_path = PathBuf::from("assets/images/img.png");
    let image = image::open(&image_path).expect("Failed to open image");

    // 创建输出目录
    let image_name = image_path.file_stem().unwrap().to_str().unwrap();
    let svg_output_dir = PathBuf::from(format!("output/svgs/{}", image_name));
    fs::create_dir_all(&svg_output_dir).expect("Failed to create SVG output directory");

    // 处理图片并提取HSV色块区域
    let mut mappings = Vec::new();
    for (name, color_model) in &color_model_mapping.colors {
        let contours = extract_hsv_regions(&image, color_model);
        let svg_file = generate_svg(contours.clone(), &svg_output_dir, name);
        let svg_path = format!("{}/{}", svg_output_dir.display(), svg_file);
        println!("Saved SVG for {} to {}", name, svg_path);

        // 保存映射关系
        mappings.push(Mapping {
            color: name.clone(),
            model_path: color_model.model_path.clone(),
            svg_image_path: svg_path,
            png_image_path: String::new(), // 不再生成PNG文件
        });
    }

    // 将映射关系保存为JSON文件
    let mappings_json = json!(mappings);
    let mappings_path = PathBuf::from("output/mappings/mappings.json");
    fs::write(mappings_path, serde_json::to_string_pretty(&mappings_json).expect("Failed to serialize mappings"))
        .expect("Failed to write mappings file");


    // 读取 mappings.json 文件
    let mappings_data = fs::read_to_string("output/mappings/mappings.json").expect("Failed to read mappings.json");
    let mappings_json: Value = serde_json::from_str(&mappings_data).expect("Failed to parse mappings.json");

    // 手动解析 JSON 数据
    let mappings: Vec<Mapping> = mappings_json.as_array().expect("Expected an array")
        .iter()
        .map(|item| {
            Mapping {
                color: item["color"].as_str().expect("Expected a string").to_string(),
                model_path: item["model_path"].as_str().expect("Expected a string").to_string(),
                svg_image_path: item["svg_image_path"].as_str().expect("Expected a string").to_string(),
                png_image_path: item["png_image_path"].as_str().expect("Expected a string").to_string(),
            }
        })
        .collect();

    // 合并所有模型文件内容
    let mut combined_obj_data = String::new();
    let mut vertex_offset = 0;

    // 前面的部分不变，我们从保存映射关系的部分开始修改
    for mapping in mappings {
        let model_data = fs::read_to_string(&mapping.model_path).expect("Failed to read model file");
        let svg_data = fs::read_to_string(&mapping.svg_image_path).expect("Failed to read SVG file");
        let svg_center = calculate_svg_center(&svg_data);

        for line in model_data.lines() {
            if line.starts_with("v ") {
                let mut coords: Vec<f32> = line[2..]
                    .split_whitespace()
                    .map(|v| v.parse().unwrap())
                    .collect();
                // 修改模型坐标以匹配SVG中心点的位置
                coords[0] += svg_center.0;
                coords[1] += svg_center.1;
                combined_obj_data.push_str(&format!("v {} {} {}\n", coords[0], coords[1], coords[2]));
            } else if line.starts_with("f ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                combined_obj_data.push_str("f ");
                for i in 1..parts.len() {
                    let vertex_index: usize = parts[i].parse().expect("Failed to parse vertex index");
                    combined_obj_data.push_str(&(vertex_index + vertex_offset).to_string());
                    combined_obj_data.push(' ');
                }
                combined_obj_data.push('\n');
            }
        }
        vertex_offset += model_data.lines().filter(|line| line.starts_with("v ")).count();
    }


    // 将合并后的数据写入到 city_model.obj 文件中
    let output_path = "output/3d_models/city_model.obj";
    let mut output_file = BufWriter::new(fs::File::create(output_path).expect("Failed to create output OBJ file"));
    output_file.write_all(combined_obj_data.as_bytes()).expect("Failed to write combined OBJ data");
}
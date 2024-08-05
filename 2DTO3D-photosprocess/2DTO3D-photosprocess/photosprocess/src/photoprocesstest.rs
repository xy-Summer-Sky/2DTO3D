use std::fs::File;
use std::path::PathBuf;
use std::collections::HashMap;
use serde::Deserialize;
use image::{DynamicImage, GenericImageView, GrayImage, Luma, Rgb};
use imageproc::edges::canny;
use imageproc::definitions::Image;
use imageproc::map::map_colors;
use imageproc::pixelops::interpolate;
use imageproc::rect::Rect;
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use std::fs;
use std::io::BufReader;

#[derive(Deserialize)]
struct ColorModel {
    hsv_range: [[u8; 3]; 2],
}

#[derive(Deserialize)]
struct ColorModelMapping {
    colors: HashMap<String, ColorModel>,
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

fn extract_hsv_regions(image: &DynamicImage, color_model: &ColorModel) -> Image<Luma<u8>> {
    let hsv_image = image.to_rgb8();
    let (width, height) = hsv_image.dimensions();
    let mut mask = GrayImage::new(width, height);

    for (x, y, pixel) in hsv_image.enumerate_pixels() {
        let hsv = convert_rgb_to_hsv(pixel);
        if hsv >= color_model.hsv_range[0] && hsv <= color_model.hsv_range[1] {
            mask.put_pixel(x, y, Luma([255]));
        } else {
            mask.put_pixel(x, y, Luma([0]));
        }
    }

    canny(&mask, 50.0, 100.0)
}

fn main() {
    // 加载颜色模型映射
    let file = File::open("config/color_model_mapping.json").expect("Failed to open color model mapping file");
    let reader = BufReader::new(file);
    let color_model_mapping: ColorModelMapping = serde_json::from_reader(reader).expect("Failed to parse color model mapping");

    // 读取图片
    let image_path = PathBuf::from("assets/images/img.png");
    let image = image::open(&image_path).expect("Failed to open image");

    // 处理图片并提取HSV色块区域
    for (name, color_model) in &color_model_mapping.colors {
        let hsv_regions = extract_hsv_regions(&image, color_model);
        let output_path = format!("output/{}_hsv_regions.png", name);
        hsv_regions.save(&output_path).expect("Failed to save HSV regions image");
        println!("Saved HSV regions for {} to {}", name, output_path);
    }
}
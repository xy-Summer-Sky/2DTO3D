use std::path::Path;

// image_processing.rs
use image::{DynamicImage, GenericImageView, GrayImage, Luma, Rgb};
use imageproc::edges::canny;
use svg::node::element::path::Data;
use svg::node::element::Path as SvgPath;
use svg::Document;

use crate::colormodel::ColorModel;

pub fn convert_rgb_to_hsv(rgb: &Rgb<u8>) -> [u8; 3] {
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

pub fn extract_contours(image: &DynamicImage, color_model: &ColorModel) -> GrayImage {
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

pub fn save_contours_as_svg(contours: &GrayImage, output_path: &Path) {
    let mut data = Data::new();
    for (x, y, pixel) in contours.enumerate_pixels() {
        if pixel[0] > 0 {
            data = data.move_to((x, y)).line_by((1, 0));
        }
    }

    let path = SvgPath::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("d", data);
    let document = Document::new()
        .set("viewBox", (0, 0, contours.width(), contours.height()))
        .add(path);
    svg::save(output_path, &document).unwrap();
}

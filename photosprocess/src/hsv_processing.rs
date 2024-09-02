// hsv_processing.rs
use crate::colormodel::ColorModel;
use image::{DynamicImage, GenericImageView, ImageBuffer, Luma, Rgb};
use imageproc::edges::canny;
use std::path::Path;
use svg::node::element::path::Data;
use svg::node::element::Path as SvgPath;
use svg::Document;

pub struct HSVProcessing;

fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;
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
    (h, s, v)
}

impl HSVProcessing {
    pub fn new() -> Self {
        HSVProcessing {}
    }

    pub fn extract_contours(
        &self,
        image: &DynamicImage,
        color_model: &ColorModel,
    ) -> ImageBuffer<Luma<u8>, Vec<u8>> {
        let hsv_image = image.to_rgb8();
        let (width, height) = hsv_image.dimensions();
        let mut mask = ImageBuffer::new(width, height);

        for (x, y, pixel) in hsv_image.enumerate_pixels() {
            let (h, s, v) = rgb_to_hsv(pixel[0], pixel[1], pixel[2]);
            if h >= color_model.hsv_range[0][0] as f32
                && h <= color_model.hsv_range[1][0] as f32
                && s >= color_model.hsv_range[0][1] as f32 / 255.0
                && s <= color_model.hsv_range[1][1] as f32 / 255.0
                && v >= color_model.hsv_range[0][2] as f32 / 255.0
                && v <= color_model.hsv_range[1][2] as f32 / 255.0
            {
                mask.put_pixel(x, y, Luma([255]));
            } else {
                mask.put_pixel(x, y, Luma([0]));
            }
        }

        canny(&mask, 50.0, 100.0)
    }

    pub fn save_contours_as_svg(
        &self,
        contours: &ImageBuffer<Luma<u8>, Vec<u8>>,
        output_path: &Path,
    ) {
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
}

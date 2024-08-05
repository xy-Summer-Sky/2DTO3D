use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct SVGGenerator;

impl SVGGenerator {
    pub fn new() -> Self {
        SVGGenerator {}
    }

    pub fn generate_svg(&self, contours: Vec<Vec<(i32, i32)>>, output_dir: &Path) -> Vec<String> {
        let mut svg_files = Vec::new();
        for (index, contour) in contours.iter().enumerate() {
            let file_name = format!("contour_{}.svg", index);
            let file_path = output_dir.join(&file_name);
            let mut file = File::create(&file_path).expect("Unable to create file");
            let mut svg_data = String::from("<svg xmlns=\"http://www.w3.org/2000/svg\">");
            svg_data.push_str("<path d=\"M ");
            for (x, y) in contour {
                svg_data.push_str(&format!("{},{} ", x, y));
            }
            svg_data.push_str("\" fill=\"none\" stroke=\"black\"/></svg>");
            file.write_all(svg_data.as_bytes()).expect("Unable to write data");
            svg_files.push(file_name);
        }
        svg_files
    }
}
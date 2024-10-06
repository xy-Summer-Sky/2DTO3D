use crate::models::request_models_dto::ExtractContourRequestData;
use base64::Engine;
use opencv::core::CV_8U;
use opencv::core::{in_range, MatTrait, Point, Scalar, Vector, BORDER_CONSTANT};
use opencv::imgcodecs::{imencode, imread, imwrite, IMREAD_COLOR};
use opencv::imgproc::flood_fill;
use opencv::imgproc::*;
use opencv::{core::Mat, core::Point2d, prelude::*};
use serde_derive::Serialize;
use std::clone::Clone;
use std::fs::File;

#[derive(Serialize)]
struct Point3d {
    x: f64,
    y: f64,
    z: f64,
}

impl From<Point> for Point3d {
    fn from(point: Point) -> Self {
        Point3d {
            x: point.x as f64,
            y: point.y as f64,
            z: 100.0, // 或者你想要的默认值
        }
    }
}
struct Triangle {
    p1: Point2d,
    p2: Point2d,
    p3: Point2d,
}
struct PolygonResult {
    convex: Vec<Point2d>,
    triangles: Vec<Triangle>,
}
pub struct ExtractContour {
    g_window_wh: [f64; 2],         // 窗口宽高
    g_location_win: [f64; 2],      // 相对于大图，窗口在图片中的位置
    g_zoom: f64,                   // 图片缩放比例
    g_step: f64,                   // 缩放系数
    g_image_original: Option<Mat>, // 原始图片
    g_image_zoom: Option<Mat>,     // 缩放后的图片
    g_image_show: Option<Mat>,     // 实际显示的图片
    mask_original: Option<Mat>,    // 用于图像处理的掩码
    p: f64,
    image_svg_model_click_positions_name: String,
}

impl ExtractContour {
    fn check_point2d(&self, i: usize, p2: &Point2d, points: &[Point2d]) -> bool {
        let length = points.len();
        let (p1, p3) = if i == 1 {
            (&points[length - 1], &points[i])
        } else if i == length {
            (&points[i - 2], &points[0])
        } else {
            (&points[i - 2], &points[i])
        };

        let v1 = Point2d {
            x: p2.x - p1.x,
            y: p2.y - p1.y,
        };
        let v2 = Point2d {
            x: p2.x - p1.x,
            y: p2.y - p1.y,
        };

        let z = v1.x * v2.y - v1.y * v2.x;

        if z < 0.0 {
            println!("点:({}, {})是凸点！", p2.x, p2.y);
            true
        } else if z == 0.0 {
            println!("点:({}, {})是平点！", p2.x, p2.y);
            true
        } else {
            println!("点:({}, {})是凹点！", p2.x, p2.y);
            false
        }
    }

    fn is_point2d_inside_triangle(
        &self,
        p0: &Point2d,
        p1: &Point2d,
        p2: &Point2d,
        p3: &Point2d,
    ) -> bool {
        fn sign(p1: &Point2d, p2: &Point2d, p3: &Point2d) -> f64 {
            (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y)
        }

        let d1 = sign(p0, p1, p2);
        let d2 = sign(p0, p2, p3);
        let d3 = sign(p0, p3, p1);

        let has_neg = (d1 < 0.0) || (d2 < 0.0) || (d3 < 0.0);
        let has_pos = (d1 > 0.0) || (d2 > 0.0) || (d3 > 0.0);

        !(has_neg && has_pos)
    }

    fn cut_polygon(&mut self, points: &mut Vec<Point2d>) -> PolygonResult {
        let mut is_convex = true;
        let mut convex_points: Vec<usize> = Vec::new(); // 凸点数组
        for i in 1..=points.len() {
            let p2 = &points[i - 1];

            if !self.check_point2d(i, p2, points) {
                is_convex = false;
            } else {
                convex_points.push(i);
            }
        }

        let mut result = PolygonResult {
            convex: Vec::new(),
            triangles: Vec::new(),
        };

        if is_convex {
            result.convex = points.clone();
            println!("是凸多边形");
            return result;
        } else {
            println!("是凹多边形");
            for &Point2d in &convex_points {
                let (p1, p2, p3): (Point2d, Point2d, Point2d);
                let length = points.len();
                let (p1_pos, p2_pos, p3_pos);
                if Point2d == 1 {
                    p1 = points[length - 1].clone();
                    p2 = points[0].clone();
                    p3 = points[1].clone();
                    p1_pos = length as isize - 1;
                    p2_pos = 0;
                    p3_pos = 1;
                } else if Point2d == length {
                    p1 = points[length - 2].clone();
                    p2 = points[length - 1].clone();
                    p3 = points[0].clone();
                    p1_pos = length as isize - 2;
                    p2_pos = length as isize - 1;
                    p3_pos = 0;
                } else {
                    p1 = points[Point2d - 2].clone();
                    p2 = points[Point2d - 1].clone();
                    p3 = points[Point2d].clone();
                    p1_pos = Point2d as isize - 2;
                    p2_pos = Point2d as isize - 1;
                    p3_pos = Point2d as isize;
                }

                let mut conflict = false;
                for j in 0..length {
                    if j as isize != p1_pos && j as isize != p2_pos && j as isize != p3_pos {
                        if self.is_point2d_inside_triangle(&points[j], &p1, &p2, &p3) {
                            conflict = true;
                        }
                    }
                }

                if conflict {
                    println!("{:?} 不是可划分点", p2);
                } else {
                    println!("{:?} 是可划分点", p2);
                    let tri_list = Triangle { p1, p2, p3 };
                    let mut new_points: Vec<Point2d> = points.to_vec();
                    new_points.remove(p2_pos as usize);
                    let result2 = self.cut_polygon(&mut new_points);
                    result.convex = result2.convex;
                    result.triangles = result2.triangles;
                    result.triangles.push(tri_list);
                    break;
                }
            }
        }

        result
    }
    fn find_pos(&self, substring: &str, vcontent: &str) -> isize {
        let v_arr: Vec<&str> = vcontent.split("\nv ").collect();
        for (i, v_item) in v_arr.iter().enumerate() {
            if substring == *v_item {
                return i as isize;
            }
        }
        -1
    }

    fn find_contour_outline(&self, image: &Mat) -> Vec<Point> {
        use opencv::core::{Vector, VectorToVec};
        use opencv::imgproc::{approx_poly_dp, arc_length, contour_area, find_contours};

        // 找到二值图像的轮廓和层级结构
        let mut contours: Vector<Vector<Point>> = Vector::new();
        find_contours(image, &mut contours, 3, 2, Point::new(0, 0)).unwrap();

        // 找到图像最外层轮廓的索引
        let max_contour_idx = contours
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| {
                contour_area(a, false)
                    .unwrap()
                    .partial_cmp(&contour_area(b, false).unwrap())
                    .unwrap()
            })
            .map(|(i, _)| i)
            .unwrap();

        // 获取最外层轮廓的点集
        let contour: Vector<Point> = contours.get(max_contour_idx).unwrap();
        println!("原始轮廓与点集数量：");
        println!("{}", contour.len());

        // 根据轮廓面积降采样点集，保证点数量最少
        let perimeter = arc_length(&contour, true).unwrap();
        let epsilon = 0.002 * perimeter;
        let mut approx: Vector<Point> = Vector::new();
        approx_poly_dp(&contour, &mut approx, epsilon, true).unwrap();

        // 将点集按顺序连接成封闭轮廓
        let mut contour_points: Vec<Point> = approx.to_vec();
        contour_points.push(contour_points[0]);

        contour_points
    }

    fn right_click(&mut self, x: f64, y: f64) -> Result<(), opencv::Error> {
        use opencv::core::{Mat, Rect, Scalar};
        if let Some(ref mut g_image_zoom) = self.g_image_zoom {
            let (g_original_h, g_original_w) = (g_image_zoom.rows(), g_image_zoom.cols());
            Mat::zeros(g_original_h + 2, g_original_w + 2, CV_8U)?;
            let mut roi = Rect::new(150, 150, 100, 100); // x, y, width, height
                                                         // let mut sub_mat: Mat = mask.roi(roi).unwrap().to_mat()?;
                                                         // sub_mat.set_to(&Scalar::from(0.0), &Mat::default())?;
            flood_fill(
                g_image_zoom,
                Point::new(
                    (self.g_location_win[0] + x) as i32,
                    (self.g_location_win[1] + y) as i32,
                ),
                Scalar::new(255.0, 0.0, 0.0, 0.0),
                &mut roi,
                Scalar::new(30.0, 30.0, 30.0, 0.0),
                Scalar::new(30.0, 30.0, 30.0, 0.0),
                FLOODFILL_FIXED_RANGE,
            )?;

            let scale =
                g_image_zoom.cols() as f64 / self.g_image_original.as_ref().unwrap().cols() as f64;
            let original_x = (self.g_location_win[0] + x) / scale;
            let original_y = (self.g_location_win[1] + y) / scale;

            let mut min_dist = f64::INFINITY;
            let mut nearest_point = Point::new(0, 0);
            for i in 0.max((original_x.floor() as i32) - 1)
                ..((original_x.ceil() as i32) + 1).min(g_image_zoom.cols())
            {
                for j in 0.max((original_y.floor() as i32) - 1)
                    ..((original_y.ceil() as i32) + 1).min(g_image_zoom.rows())
                {
                    let dist =
                        ((original_x - i as f64).powi(2) + (original_y - j as f64).powi(2)).sqrt();
                    if dist < min_dist {
                        min_dist = dist;
                        nearest_point = Point::new(i, j);
                    }
                }
            }

            self.mask_original = Mat::zeros(
                self.g_image_original.as_ref().unwrap().rows() + 2,
                self.g_image_original.as_ref().unwrap().cols() + 2,
                CV_8U,
            )?
            .to_mat()
            .ok();
            // 进行其他操作，例如使用 set_to 方法
            // sub_mat.set_to(&Scalar::all(0.0), &Mat::default())?;
            let mut rect = Rect::new(
                0,
                0,
                self.g_image_original.as_ref().unwrap().cols() + 2,
                self.g_image_original.as_ref().unwrap().rows() + 2,
            );
            flood_fill(
                self.g_image_original.as_mut().unwrap(),
                nearest_point,
                Scalar::new(255.0, 0.0, 0.0, 0.0),
                &mut rect,
                Scalar::new(30.0, 30.0, 30.0, 0.0),
                Scalar::new(30.0, 30.0, 30.0, 0.0),
                FLOODFILL_FIXED_RANGE,
            )?;
        } else {
            println!("g_image_zoom is None");
        }

        Ok(())
    }

    fn process_image(
        &mut self,
        json_data: &serde_json::Value,
        image_data:&String
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        use opencv::core::MatTrait;
        println!("g_window_wh: {:?}", self.g_window_wh);
        let mut contourn_points = serde_json::json!({
            "city_id": json_data["city_id"],
            "contours": [],
            "image_width": self.g_window_wh[0],
            "image_height": self.g_window_wh[1] ,
        });

        let click_locations = json_data["right_clicks"].as_array().unwrap();
        // let image_path = json_data["image_path"].as_str().unwrap();
        // self.g_image_original = Some(opencv::imgcodecs::imdecode(&opencv::core::Vector::from(image_data), IMREAD_COLOR)?);
        self.g_image_zoom = self.g_image_original.clone();

        // let g_window_name = "contourImg";
        // named_window(g_window_name, WINDOW_NORMAL)?;
        // resize_window(
        //     g_window_name,
        //     self.g_window_wh[0] as i32,
        //     self.g_window_wh[1] as i32,
        // )?;

        // let file = File::create("newoutput2.json")?;
        // serde_json::to_writer(&file, &json_data)?;

        // let image_data=json_data["image_data"]. as_str().unwrap();
        for click_location in click_locations {
            //每次处理一个点击位置，都重新加载一次原始图片
            self.g_image_original = Some(opencv::imgcodecs::imdecode(
                &opencv::core::Vector::<u8>::from(
                    base64::engine::general_purpose::STANDARD.decode(&image_data)?,
                ),
                IMREAD_COLOR,
            )?);
            self.g_image_zoom = self.g_image_original.clone();
            self.right_click(
                click_location["x"].as_f64().unwrap(),
                click_location["y"].as_f64().unwrap(),
            )?;

            // if self.g_image_original.as_ref().unwrap().empty() {
            //     return Err(Box::new(std::io::Error::new(
            //         std::io::ErrorKind::NotFound,
            //         format!(
            //             "Image at path {} not found or could not be loaded.",
            //             image_path
            //         ),
            //     )));
            // }

            let g_image_original = self.g_image_original.as_mut().unwrap();
            let mut mask = Mat::default();
            in_range(
                g_image_original,
                &Scalar::new(255.0, 0.0, 0.0, 0.0),
                &Scalar::new(255.0, 0.0, 0.0, 0.0),
                &mut mask,
            )?;

            g_image_original.set_to(&Scalar::new(255.0, 255.0, 255.0, 0.0), &mask)?;
            let mut inverted_mask = Mat::default();
            opencv::core::bitwise_not(&mask, &mut inverted_mask, &Mat::default())?;
            g_image_original.set_to(&Scalar::new(0.0, 0.0, 0.0, 0.0), &inverted_mask)?;

            let mut buf = Vector::new();
            imencode(
                ".png",
                &self.g_image_original.as_ref().unwrap(),
                &mut buf,
                &Vector::new(),
            )?;
            let image = opencv::imgcodecs::imdecode(&buf, IMREAD_COLOR)?;
            let mut gray = Mat::default();
            cvt_color(&image, &mut gray, COLOR_BGR2GRAY, 0)?;
            let kernel = Mat::ones(5, 5, CV_8U)?;
            let mut dilation = Mat::default();
            {
                let gray_ref = &gray;
                dilate(
                    gray_ref,
                    &mut dilation,
                    &kernel,
                    Point::new(-1, -1),
                    1,
                    BORDER_CONSTANT,
                    Scalar::default(),
                )?;
                let mut temp_image = dilation.clone(); // 创建一个临时副本
                erode(
                    &dilation,
                    &mut temp_image,
                    &kernel,
                    Point::new(-1, -1),
                    1,
                    BORDER_CONSTANT,
                    Scalar::default(),
                )?;
                dilation = temp_image;
            }

            imwrite("dilation.png", &dilation, &Vector::new())?;

            let image = imread("dilation.png", IMREAD_COLOR)?;
            let mut gray = Mat::default();
            cvt_color(&image, &mut gray, COLOR_BGR2GRAY, 0)?;

            let mut image = Mat::default();
            threshold(&gray, &mut image, 127.0, 255.0, THRESH_BINARY)?;

            let contour: Vec<Point> = self.find_contour_outline(&image);
            println!("失真程度为0.002*周长以内的点结果");
            for Point in &contour {
                println!("[{}, {}],", Point.x, Point.y);
            }
            println!("{}", contour.len());

            let middle_part = &contour[..contour.len() - 1];
            let n = 16 % middle_part.len();
            let contour = [&middle_part[n..], &middle_part[..n]].concat();
            let contour: Vec<Point> = [&contour[..], &vec![contour[0]]].concat();

            let mut points: Vec<Point3d> = contour
                .iter()
                .map(|p: &Point| Point3d {
                    x: p.x as f64,
                    y: p.y as f64,
                    z: 0.0,
                })
                .collect();
            points.pop();
            for Point2d in &mut points {
                Point2d.z = 100.0;
            }

            // if click_location["type"] == "parent" {
            //     contourn_points["parent"]["contour_Points"] = serde_json::to_value(&points)?;
            // } else if click_location["type"] == "children" {
            //     contourn_points["children"]
            //         .as_array_mut()
            //         .unwrap()
            //         .push(serde_json::json!({
            //             "contour_Points": points,
            //             "height": 100
            //         }));
            // }
            contourn_points["contours"]
                .as_array_mut()
                .unwrap()
                .push(serde_json::json!({
                    "contour_points": points,
                    "height": 100
                }));
        }

        // let file = File::create(output_path)?;
        // serde_json::to_writer_pretty(&file, &contourn_points)?;

        Ok(contourn_points)
    }

    pub fn extract_contour_api(
        &mut self,
        image_data: &ExtractContourRequestData,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        std::env::current_dir()?;
        self.g_image_original = Some(opencv::imgcodecs::imdecode(
            &opencv::core::Vector::<u8>::from(
                base64::engine::general_purpose::STANDARD.decode(&image_data.image_data)?,
            ),
            IMREAD_COLOR,
        )?);
        self.g_window_wh = [
            self.g_image_original.as_ref().unwrap().cols() as f64,
            self.g_image_original.as_ref().unwrap().rows() as f64,
        ]; // 窗口宽高

        //输出宽高
        println!("窗口宽高：{}, {}", self.g_window_wh[0], self.g_window_wh[1]);
           // let file = File::open(json_path)?;
        let json_data = serde_json::to_value(&image_data)?;
        // self.g_location_win = [0.0, 0.0];

        Ok(self.process_image(&json_data,&image_data.image_data)?)
    }
    pub fn new() -> Self {
        ExtractContour {
            g_window_wh: [0.0, 0.0],
            g_location_win: [0.0, 0.0],
            g_zoom: 1.0,
            g_step: 0.1,
            g_image_original: None,
            g_image_zoom: None,
            g_image_show: None,
            mask_original: None,
            p: 0.0,
            image_svg_model_click_positions_name: String::new(),
        }
    }

    // pub fn new_model_and_receive_svg(
    //     &mut self,
    //     city_id: &str,
    //     svg_data: &[u8],
    // ) -> Result<(), Box<dyn std::error::Error>> {
    //     use std::fs;
    //
    //     // Create the base directory for the city
    //     let base_dir = format!("data/{}", city_id);
    //     fs::create_dir_all(&base_dir)?;
    //
    //     // Create the SVG directory
    //     let svg_dir = format!("{}/svg", base_dir);
    //     fs::create_dir_all(&svg_dir)?;
    //
    //     // Save the SVG data to a file
    //     let svg_path = format!("{}/model.svg", svg_dir);
    //     fs::write(&svg_path, svg_data)?;
    //
    //     // Process the SVG data (this is a placeholder, replace with actual processing logic)
    //     println!("SVG data received and saved to {}", svg_path);
    //
    //     Ok(())
    // }

    // fn save_contours_as_svg(&self, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    //     use std::fs::File;
    //     use std::io::Write;
    //
    //     let mut svg_content =
    //         String::from(r#"<svg viewBox="0 0 300 300" xmlns="http://www.w3.org/2000/svg">"#);
    //
    //     // 添加父轮廓
    //     svg_content.push_str(r#"<path d="M0,0 L0,199 L189,199 L189,0 z" fill="none" id="parent" stroke="blue" stroke-width="2"/>"#);
    //
    //     // 添加子轮廓
    //     svg_content.push_str(r#"<path d="M82,21 L76,29 L73,41 L72,53 L68,64 L61,69 L54,69 L47,67 L47,167 L95,138 L103,139 L146,167 L146,66 L99,20 L86,19 z" fill="none" id="child1" stroke="red" stroke-width="2"/>"#);
    //
    //     svg_content.push_str("</svg>");
    //
    //     let mut file = File::create(output)?;
    //     file.write_all(svg_content.as_bytes())?;
    //
    //     Ok(())
    // }

    // //命名使用的是获取当前日期时间+Session的ID+文件名称
    // fn get_unique_filename(base_path: &str, file_name: &str, extension: &str) -> String {
    //     let now = chrono::Utc::now();
    //     let unique_path = format!(
    //         "{}-{}-{}-{}.{}",
    //         now.year(),
    //         now.month(),
    //         now.day(),
    //         now.timestamp_subsec_millis(),
    //         extension
    //     );
    //     unique_path
    // }
    // pub fn create_city_and_extract_contours(
    //     &mut self,
    //     image_data: ExtractContourRequestData,
    //     // json_path: &str,
    //     output: &str,
    // ) -> Result<(), Box<dyn std::error::Error>> {
    //     self.extract_contour_api(image_data,  output)?;
    //     self.save_contours_as_svg(output)?;
    //     Ok(())
    // }
}

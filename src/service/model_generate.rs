use delaunator::triangulate;
use geo_types::point;
use nalgebra::Point2;
use std::fs::File;
use std::io::Write;

pub struct ModelGenerate {
    title_content: String,
    v_content: String,
    vertex2num_dict: std::collections::HashMap<String, usize>,
    vertex_count: usize,
    vt_content: String,
    vn_content: String,
    vnormal_count: usize,
    face_content: String,
    side_face_content: String,
    top_face_content: String,
    bottom_face_content: String,
}

impl ModelGenerate {
    pub(crate) fn new() -> Self {
        Self {
            title_content: String::from("mtllib Building1.mtl\no Building1"),
            v_content: String::new(),
            vertex2num_dict: std::collections::HashMap::new(),
            vertex_count: 0,
            vt_content: String::from("\nvt 0.625000 0.500000"),
            vn_content: String::from("\nvn 0 1 0\nvn 0 -1 0"),
            vnormal_count: 0,
            face_content: String::from("\ng box_Cube\nusemtl Material01\ns off"),
            side_face_content: String::new(),
            top_face_content: String::new(),
            bottom_face_content: String::new(),
        }
    }

    fn gen_contour_vertex(&mut self, contour: &Vec<(f64, f64)>, height: f64) {
        for point in contour {
            let lon = point.0;
            let lat = point.1;
            self.v_content
                .push_str(&format!("\nv {} {} {}", lon, height, lat));
            self.vertex2num_dict
                .insert(format!("v {} {} {}", lon, height, lat), self.vertex_count);
            self.vertex_count += 1;
        }
    }

    fn gen_side_face(&mut self, contour: &Vec<(f64, f64)>, height_self: f64, height_target: f64) {
        for i in 1..=contour.len() {
            let point1 = contour[i - 1];
            let point2 = contour[i - 1];
            let point3 = if i < contour.len() {
                contour[i]
            } else {
                contour[0]
            };
            let point4 = if i < contour.len() {
                contour[i]
            } else {
                contour[0]
            };
            let a = (0.0 - height_self) * (point3.1 - point1.1)
                - (0.0 - height_self) * (point2.1 - point1.1);
            let b = 0.0;
            let c = (point2.0 - point1.0) * (0.0 - height_self)
                - (point3.0 - point1.0) * (0.0 - height_self);
            self.vn_content.push_str(&format!("\nvn {} {} {}", a, b, c));
            self.vnormal_count += 1;
            let p1_substr = format!("v {} {} {}", point1.0, height_self, point1.1);
            let p2_substr = format!("v {} {} {}", point2.0, height_target, point2.1);
            let p3_substr = format!("v {} {} {}", point3.0, height_target, point3.1);
            let p4_substr = format!("v {} {} {}", point4.0, height_self, point4.1);
            let p1_num = self.vertex2num_dict[&p1_substr];
            let p2_num = self.vertex2num_dict[&p2_substr];
            let p3_num = self.vertex2num_dict[&p3_substr];
            let p4_num = self.vertex2num_dict[&p4_substr];
            self.side_face_content
                .push_str(&format!("\nf {} {} {} {}", p1_num, p2_num, p3_num, p4_num));
        }
    }

    pub fn gen_top_face(
        &mut self,
        contour: &Vec<(f64, f64)>,
        child_contours: &Vec<Vec<(f64, f64)>>,
        height: f64,
    ) {
        let mut points: Vec<delaunator::Point> = contour
            .iter()
            .map(|&(x, y)| delaunator::Point { x, y })
            .collect();
        let result = triangulate(&mut points); // Assuming triangulate does not return an error.

        let mut contour_paths = Vec::new();
        for child_contour in child_contours {
            let path: Vec<Point2<f64>> = child_contour
                .iter()
                .map(|&(x, y)| Point2::new(x, y))
                .collect();
            contour_paths.push(path);
        }

        let contour_path2: Vec<Point2<f64>> =
            contour.iter().map(|&(x, y)| Point2::new(x, y)).collect();

        let tri_centers: Vec<Point2<f64>> = result
            .triangles
            .chunks(3)
            .map(|tri| {
                let p1 = &points[tri[0]];
                let p2 = &points[tri[1]];
                let p3 = &points[tri[2]];
                Point2::new((p1.x + p2.x + p3.x) / 3.0, (p1.y + p2.y + p3.y) / 3.0)
            })
            .collect();

        let mut is_inside = vec![true; tri_centers.len()];
        for path in &contour_paths {
            for (i, center) in tri_centers.iter().enumerate() {
                if !path.contains(center) {
                    is_inside[i] = false;
                }
            }
        }

        let mut is_inside2 = vec![true; tri_centers.len()];
        for (i, center) in tri_centers.iter().enumerate() {
            if !contour_path2.contains(center) {
                is_inside2[i] = false;
            }
        }

        for (i, tri) in result.triangles.chunks(3).enumerate() {
            if !is_inside[i] || !is_inside2[i] {
                continue;
            }
            self.top_face_content.push_str("\nf ");
            let tri_reverse = [tri[2], tri[1], tri[0]];
            for &index in &tri_reverse {
                let point = &points[index];
                let substring = format!("v {} {} {}", point.x, height, point.y);
                if let Some(&num) = self.vertex2num_dict.get(&substring) {
                    self.top_face_content.push_str(&format!("{} ", num + 1));
                } else {
                    println!("构造三角形的时候，出现了不存在的点");
                }
            }
        }
    }

    pub fn gen_top_face2(&mut self, contour: &Vec<(f64, f64)>, height: f64) {
        use delaunator::{triangulate, Point as DelaunatorPoint};
        use geo::{algorithm::contains::Contains, Point as GeoPoint, Polygon};
        use geo_types::{Coord, LineString};
        use itertools::Itertools;
        // 将 (f64, f64) 元组转换为 Delaunator 的 Point 类型
        let points: Vec<DelaunatorPoint> = contour
            .iter()
            .map(|&(x, y)| DelaunatorPoint { x, y })
            .collect();

        // 执行三角剖分
        let delaunay = triangulate(&points);

        // 创建多边形路径，使用 geo 库的 Polygon
        let polygon: Polygon<f64> = Polygon::new(
            LineString::from(
                contour
                    .iter()
                    .map(|&(x, y)| Coord { x, y })
                    .collect::<Vec<_>>(),
            ),
            vec![],
        );

        // 计算三角形质心并筛选内部的三角形
        let triangles_centers = delaunay
            .triangles
            .iter()
            .tuples()
            .map(|(&i1, &i2, &i3)| {
                let p1 = &points[i1];
                let p2 = &points[i2];
                let p3 = &points[i3];
                GeoPoint::new((p1.x + p2.x + p3.x) / 3.0, (p1.y + p2.y + p3.y) / 3.0)
            })
            .collect::<Vec<_>>();

        // 过滤出所有中心点在多边形内的三角形
        let is_inside = triangles_centers
            .iter()
            .map(|center| polygon.contains(center))
            .collect::<Vec<bool>>();

        // 构建面内容
        for (i, t) in delaunay.triangles.iter().tuples().enumerate() {
            let index: usize = i;
            let tri: (&usize, &usize, &usize) = t;
            if !is_inside[index] {
                continue;
            }
            self.top_face_content.push_str("\nf ");
            let tri_reverse = [tri.2, tri.1, tri.0];
            for &index in &tri_reverse {
                let point: &delaunator::Point = &points[*index];
                let substring = format!("v {} {} {}", point.x, height, point.y);
                if let Some(&num) = self.vertex2num_dict.get(&substring) {
                    self.top_face_content.push_str(&format!("{} ", num + 1));
                } else {
                    println!("构造三角形的时候，出现了不存在的点");
                }
            }
        }
    }

    pub fn gen_bottom_face(&mut self, contour: &[(f64, f64)], height: f64) {
        // 转换点为 Delaunator 所需的格式
        use delaunator::{triangulate, Point as DelaunatorPoint};
        use geo::{algorithm::contains::Contains, Polygon};
        use geo_types::Coord;
        use itertools::Itertools;
        let points: Vec<DelaunatorPoint> = contour
            .iter()
            .map(|&(x, y)| DelaunatorPoint { x, y })
            .collect();

        // 执行三角剖分
        let delaunay = triangulate(&points);

        // 创建一个多边形来检查点是否在内部
        let polygon: Polygon<f64> = Polygon::new(
            contour
                .iter()
                .map(|&(x, y)| Coord { x, y })
                .collect::<Vec<_>>()
                .into(),
            vec![],
        );

        // 计算三角形中心
        let centers = delaunay
            .triangles
            .iter()
            .tuples()
            .map(|(&i1, &i2, &i3)| {
                let c = vec![i1, i2, i3].iter().map(|&i| &points[i]).fold(
                    DelaunatorPoint { x: 0.0, y: 0.0 },
                    |acc, p| DelaunatorPoint {
                        x: acc.x + p.x / 3.0,
                        y: acc.y + p.y / 3.0,
                    },
                );
                point!(x: c.x, y: c.y)
            })
            .collect::<Vec<_>>();

        // 筛选出中心在多边形内的三角形索引
        let is_inside = centers
            .iter()
            .map(|center| polygon.contains(center))
            .collect::<Vec<bool>>();

        // 过滤三角形并生成底面内容
        for (i, &inside) in is_inside.iter().enumerate() {
            if inside {
                let indices = &delaunay.triangles[3 * i..3 * i + 3];
                self.bottom_face_content.push_str("\nf ");
                for &index in indices.iter().rev() {
                    // 注意逆序处理
                    let point = &points[index];
                    let substring = format!("v {} {} {}", point.x, height, point.y);
                    if let Some(&num) = self.vertex2num_dict.get(&substring) {
                        self.bottom_face_content.push_str(&format!("{} ", num));
                    } else {
                        println!("构造三角形的时候，出现了不存在的点");
                    }
                }
            }
        }
    }
    pub (crate) fn genenate_model_one_parent_contour(&mut self, contour: &Vec<(f64, f64)>, height: f64) {
        self.gen_contour_vertex(contour, height);
        self.gen_contour_vertex(contour, 0.0);
        self.gen_side_face(contour, height, 0.0);
        self.gen_top_face2(contour, height);
        self.gen_bottom_face(contour, 0.0);

    }

    pub(crate) fn generate_model(
        &mut self,
        contour: &Vec<(f64, f64)>,
        parent_contour: &Vec<(f64, f64)>,
        child_contours: &Vec<Vec<(f64, f64)>>,
        height_params: &Vec<f64>,
    ) {
        let parent_height = height_params[0];
        self.gen_contour_vertex(parent_contour, parent_height);
        self.gen_contour_vertex(parent_contour, 0.0);
        for (idx, child_contour) in child_contours.iter().enumerate() {
            self.gen_contour_vertex(child_contour, parent_height);
            self.gen_contour_vertex(child_contour, height_params[idx + 1]);
        }
        println!("{:?}", self.vertex2num_dict);
        self.gen_side_face(parent_contour, parent_height, 0.0);
        for (idx, child_contour) in child_contours.iter().enumerate() {
            self.gen_side_face(child_contour, height_params[idx + 1], parent_height);
        }

        self.gen_top_face(contour, child_contours, height_params[0]);
        for (idx, child_contour) in child_contours.iter().enumerate() {
            self.gen_top_face2(child_contour, height_params[idx + 1]);
        }
        self.gen_bottom_face(parent_contour, 0.0);
    }
    fn process_json(
        &self,
        json_data: &serde_json::Value,
    ) -> (
        Vec<(f64, f64)>,
        Vec<(f64, f64)>,
        Vec<Vec<(f64, f64)>>,
        Vec<f64>,
        String,
    ) {
        let parent_contour: Vec<(f64, f64)> = json_data["parent"]["contour_points"]
            .as_array()
            .unwrap()
            .iter()
            .map(|point| {
                (
                    point["lon"].as_f64().unwrap(),
                    point["lat"].as_f64().unwrap(),
                )
            })
            .collect();

        let child_contours: Vec<Vec<(f64, f64)>> = json_data["children"]
            .as_array()
            .unwrap()
            .iter()
            .map(|child| {
                child["contour_points"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|point| {
                        (
                            point["lon"].as_f64().unwrap(),
                            point["lat"].as_f64().unwrap(),
                        )
                    })
                    .collect()
            })
            .collect();

        let height_params: Vec<f64> =
            std::iter::once(json_data["parent"]["height"].as_f64().unwrap())
                .chain(
                    json_data["children"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|child| child["height"].as_f64().unwrap()),
                )
                .collect();
        let contours: Vec<(f64, f64)> = parent_contour
            .iter()
            .chain(child_contours.iter().flatten())
            .cloned()
            .collect();
        let save_path = "Building2-模型.obj".to_string();

        (
            contours,
            parent_contour.clone(),
            child_contours,
            height_params,
            save_path,
        )
    }
    pub fn save_model_file(&mut self, outputpath: &str) {
        // let file = File::open(inputfile).expect("Unable to open file");
        // let json_data: serde_json::Value =
        //     serde_json::from_reader(file).expect("Unable to parse JSON");
        // let (contour, parent_contour, child_contours, height_params, save_path) =
        //     self.process_json(&json_data);
        // self.generate_model(&contour, &parent_contour, &child_contours, &height_params);
        let obj_content = format!(
            "{}{}{}{}{}{}{}{}",
            self.title_content,
            self.v_content,
            self.vt_content,
            self.vn_content,
            self.face_content,
            self.top_face_content,
            self.bottom_face_content,
            self.side_face_content
        );
        println!("{}", obj_content);
        let mut obj_file = File::create(outputpath).expect("Unable to create file");
        obj_file
            .write_all(obj_content.as_bytes())
            .expect("Unable to write data");
    }
}

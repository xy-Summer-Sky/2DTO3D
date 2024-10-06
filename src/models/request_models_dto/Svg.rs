use actix_web::FromRequest;
use std::fs::File;
use std::io::Write;
// src/models/request_models_dto/Svg.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Contour {
    pub path: Vec<Point>,
    pub height: f32,
}

#[derive(Serialize, Deserialize)]
pub struct PathGroup {
    pub city_id: i32,
    pub user_id: i32,
    pub parent_contour: Contour,
    pub child_contours: Vec<Contour>,
    pub image_id:i32
}

#[derive(Serialize, Deserialize)]
pub struct PathGroups {
    pub session_id: String,
    pub path_groups: Vec<PathGroup>,
}
/// ## PathGroups Request Example
///
/// ### HTTP Request
/// `POST /build_model`
///
/// ### Headers
/// | Key           | Value               |
/// |---------------|---------------------|
/// | Content-Type  | application/json    |
/// | Cookie        | session_id=your_session_id |
///
/// ### Request Body
/// ```json
/// {
///   "session_id": "your_session_id",
///   "path_groups": [
///     {
///       "city_id": 1,
///       "user_id": 123,
///       "parent_contour": {
///         "path": [
///           {"x": 0.0, "y": 0.0},
///           {"x": 1.0, "y": 1.0}
///         ],
///         "height": 10.0
///       },
///       "child_contours": [
///         {
///           "path": [
///             {"x": 2.0, "y": 2.0},
///             {"x": 3.0, "y": 3.0}
///           ],
///           "height": 5.0
///         }
///       ],
///       "image_id": 456
///     }
///   ]
/// }
/// ```
///
/// ### Response
/// #### Success
/// ```json
/// {
///   "message": "Model built successfully"
/// }
/// ```
///
/// #### Error
/// ```json
/// {
///   "error": "Failed to build model: error_message"
/// }
/// ```
impl FromRequest for PathGroups {
    type Error = actix_web::Error;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let session_id = req
            .cookie("session_id")
            .map(|c| c.value().to_owned())
            .unwrap_or_default();
        let path_groups = Vec::new();
        futures::future::ready(Ok(PathGroups {
            session_id,
            path_groups,
        }))
    }
}

impl PathGroup {
    pub fn to_svg(&self, file: &mut File,svg_height:f64,svg_width:f64) -> std::io::Result<()> {
        writeln!(
            file,
            r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#
        )?;
      writeln!(
    file,
    r#"<svg viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg">"#,
    svg_width, svg_height
)?;

        // 写入父路径
        writeln!(
            file,
            r#"<path id="parent" d="M {} Z" fill="none" stroke="black" stroke-width="1" />"#,
            self.parent_contour
                .path
                .iter()
                .map(|p| format!("{},{}", p.x, p.y))
                .collect::<Vec<String>>()
                .join(" L ")
        )?;

        // 写入子路径
        for (index, child) in self.child_contours.iter().enumerate() {
            writeln!(
                file,
                r#"<path id="child{}" d="M {} Z" fill="none" stroke="black" stroke-width="1" />"#,
                index + 1,
                child
                    .path
                    .iter()
                    .map(|p| format!("{},{}", p.x, p.y))
                    .collect::<Vec<String>>()
                    .join(" L ")
            )?;
        }

        writeln!(file, "</svg>")?;
        Ok(())
    }
}

impl PathGroups {
    pub fn save_path_groups(&self,svg_height:f64,svg_width:f64) -> Result<(), Box<dyn std::error::Error>> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
        for (index, path_group) in self.path_groups.iter().enumerate() {
            let file_base_path = format!("data/{}/{}/svgs", path_group.user_id, path_group.city_id);
            let file_name = format!(
                "{}/path_group_{}_{}_{}.svg",
                file_base_path,
                self.session_id,
                path_group.city_id,
                timestamp + index as u64
            );
            let mut file = File::create(&file_name)?;
            path_group.to_svg(&mut file,svg_height,svg_width)?;
        }

        Ok(())
    }
}
//用来处理原始扫描出来的svg的请求结构体，包含了用户id、城市id、图片id、session_id和svg内容，sessionid用于命名
#[derive(Serialize, Deserialize)]
pub struct OriginalSvg {
    pub(crate) user_id: i32,
    pub(crate) city_id: i32,
    pub(crate) image_id: i32,
    pub(crate) session_id: String,
    pub(crate) svg_content: String,
}
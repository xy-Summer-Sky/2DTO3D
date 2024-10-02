use std::fs::File;
use actix_web::FromRequest;
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
}

pub struct PathGroups{
    pub session_id: String,
    pub path_groups: Vec<PathGroup>,
}
impl  FromRequest for PathGroups {
    type Error = actix_web::Error;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
        let session_id = req.cookie("session_id").map(|c| c.value().to_owned()).unwrap_or_default();
        let path_groups = Vec::new();
        futures::future::ready(Ok(PathGroups { session_id, path_groups }))
    }
}
impl PathGroup {
    pub fn to_svg(&self, file: &mut File) -> std::io::Result<()> {
        writeln!(file, r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#)?;
        writeln!(file, r#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1">"#)?;

        // 写入父路径
        writeln!(file, r#"<path id="parent" d="M {} Z" fill="none" stroke="black" stroke-width="1" />"#,
                 self.parent_contour.path.iter()
                     .map(|p| format!("{},{}", p.x, p.y))
                     .collect::<Vec<String>>()
                     .join(" L "))?;

        // 写入子路径
        for (index, child) in self.child_contours.iter().enumerate() {
            writeln!(file, r#"<path id="child{}" d="M {} Z" fill="none" stroke="black" stroke-width="1" />"#,
                     index + 1,
                     child.path.iter()
                         .map(|p| format!("{},{}", p.x, p.y))
                         .collect::<Vec<String>>()
                         .join(" L "))?;
        }

        writeln!(file, "</svg>")?;
        Ok(())
    }
}

impl PathGroups {
    pub fn save_path_groups(&self) -> Result<(), Box<dyn std::error::Error>> {
        let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs();
        for (index, path_group) in self.path_groups.iter().enumerate() {
            let file_base_path= format!("data/{}/{}/svgs", path_group.user_id,path_group.city_id);
            let file_name = format!("{}/path_group_{}_{}_{}.svg", file_base_path, self.session_id, path_group.city_id, timestamp + index as u64);
            let mut file = File::create(&file_name)?;
            path_group.to_svg(&mut file)?;
        }

        Ok(())
    }
}
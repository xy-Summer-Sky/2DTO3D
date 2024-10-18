use crate::dao::CityDao;
use crate::models::request_models_dto::ExtractContourRequestData;
use crate::models::ModelResponse;
use crate::pool::app_state::DbPool;
use crate::service::FileManager;
use base64::Engine;
use std::error::Error;

pub struct ModelApiIntegrate;

impl ModelApiIntegrate {
    //新建城市和目录,并获取city_id的api处于file_manager.rs
    pub async fn extract_contours(
        image_upload: &ExtractContourRequestData,
        pool: &crate::pool::app_state::DbPool,
    ) -> Result<(String, f64, f64), String> {
        // Extract contours from the uploaded image
        let mut extracted_contours = crate::service::extract_contour::ExtractContour::new();
        let contours = extracted_contours
            .extract_contour_api(image_upload)
            .expect("Failed to extract contours");
        //从contours中提取image_width和image_height和city_id
        let image_width = contours["image_width"].as_f64().unwrap();
        let image_height = contours["image_height"].as_f64().unwrap();
        let city_id = contours["city_id"].as_i64().unwrap();
        //更新数据库中的city_id的image_width和image_height
        CityDao::update_city_svg_height_and_svg_width(
            pool,
            city_id as i32,
            image_width as f32,
            image_height as f32,
        )
            .unwrap();

        //处理轮廓为svg格式，并持久化，并且做为返回值
        let svg = Self::contours_to_svg(&contours).expect("Failed to save contours as SVG");
        //返回轮廓
        Ok(svg)
    }
    //svg高和宽也是参数
    pub fn contours_to_svg(
        contourn_points: &serde_json::Value,
    ) -> Result<(String, f64, f64), Box<dyn Error>> {
        let svg_width = contourn_points["image_width"].as_f64().unwrap();
        let svg_height = contourn_points["image_height"].as_f64().unwrap();

        let mut svg_content = String::from(format!(
            r#"<svg viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg">"#,
            svg_width, svg_height
        ));

        if let Some(contours) = contourn_points["contours"].as_array() {
            for (index, contour) in contours.iter().enumerate() {
                if let Some(points) = contour["contour_points"].as_array() {
                    let path_data = points
                        .iter()
                        .enumerate()
                        .map(|(i, point)| {
                            let x = point["x"].as_f64().unwrap_or(0.0);
                            let y = point["y"].as_f64().unwrap_or(0.0);
                            if i == 0 {
                                format!("M{},{}", x, y)
                            } else {
                                format!("L{},{}", x, y)
                            }
                        })
                        .collect::<Vec<String>>()
                        .join(" ");
                    let path_element = format!(
                        r#"<path id="contour_{}" d="{} Z" fill="none" stroke="black" stroke-width="1"/>"#,
                        index, path_data
                    );
                    svg_content.push_str(&path_element);
                }
            }
        }

        svg_content.push_str("</svg>");

        Ok((svg_content, svg_width, svg_height))
    }

    //轮廓提取之后，接收用户的分组信息，将分组信息持久化到数据库中
    //每一组有一个父轮廓和许多子轮廓，每一个轮廓都有着不同的高度，提交这些信息进行模型构建
    //原本的svg中，每一个轮廓都会有一个id，用户只需要选择不同的轮廓id，就可以进行分组，标记出每一组轮廓的父轮廓和子轮廓，并且赋予每个轮廓高度参数
    //每一组轮廓都会是一个独立的svg文件，保存到目录中，并且保存到数据库记录中，并且每个svg生成独立的model
    ////use actix_web::FromRequest;
    // // use std::fs::File;
    // // use std::io::Write;
    // // // src/models/request_models_dto/Svg.rs
    // // use serde::{Deserialize, Serialize};
    // //
    // // #[derive(Serialize, Deserialize)]
    // // pub struct Point {
    // //     pub x: f32,
    // //     pub y: f32,
    // // }
    // //
    // // #[derive(Serialize, Deserialize)]
    // // pub struct Contour {
    // //     pub path: Vec<Point>,
    // //     pub height: f32,
    // // }
    // //
    // // #[derive(Serialize, Deserialize)]
    // // pub struct PathGroup {
    // //     pub city_id: i32,
    // //     pub user_id: i32,
    // //     pub parent_contour: Contour,
    // //     pub child_contours: Vec<Contour>,
    // // }
    // //
    // // pub struct PathGroups {
    // //     pub session_id: String,
    // //     pub path_groups: Vec<PathGroup>,
    // // }
    // // impl FromRequest for PathGroups {
    // //     type Error = actix_web::Error;
    // //     type Future = futures::future::Ready<Result<Self, Self::Error>>;
    // //
    // //     fn from_request(
    // //         req: &actix_web::HttpRequest,
    // //         payload: &mut actix_web::dev::Payload,
    // //     ) -> Self::Future {
    // //         let session_id = req
    // //             .cookie("session_id")
    // //             .map(|c| c.value().to_owned())
    // //             .unwrap_or_default();
    // //         let path_groups = Vec::new();
    // //         futures::future::ready(Ok(PathGroups {
    // //             session_id,
    // //             path_groups,
    // //         }))
    // //     }
    // // }

    pub async fn generate_from_path_groups(
        path_groups: &crate::models::request_models_dto::PathGroups,
        pool: &DbPool,
    ) -> Result<ModelResponse, Box<dyn Error>> {
        //检测path_groups中的子轮廓数量是否大于0
        if path_groups.path_groups.len() == 0 {
            return Err("No path groups provided".into());
        }
        //构建一个向量，存储每一个path_group的obj模型，最终做为返回值
        let mut models = Vec::new();
        let (svg_height, svg_width) =
            CityDao::get_city_svg_height_and_svg_width(pool, path_groups.path_groups[0].city_id)
                .unwrap();

        //遍历每一个path_group，生成模型
        for path_group in &path_groups.path_groups {
            let model_save_path = format!(
                "data/{}/{}/obj_models/model_{}_{}_{}.obj",
                path_group.user_id,
                path_group.city_id,
                path_groups.session_id,
                chrono::Utc::now().timestamp(),
                uuid::Uuid::new_v4()
            );
            let mut model_generate = crate::service::model_generate::ModelGenerate::new();

            //只有一个父亲轮廓的情况
            // if (path_group.child_contours.len() == 0) {
            //     let parent_contour: Vec<(f64, f64)> = path_group
            //         .parent_contour
            //         .path
            //         .iter()
            //         .map(|point| (point.x as f64, point.y as f64))
            //         .chain(std::iter::once((
            //             path_group.parent_contour.path[0].x as f64,
            //             path_group.parent_contour.path[0].y as f64,
            //         )))
            //         .collect();
            //
            //     let height_params: Vec<f64> = vec![path_group.parent_contour.height as f64];
            //     model_generate.genenate_model_one_parent_contour(&parent_contour, height_params[0]);
            // } else {
                //将父轮廓和子轮廓提取出来
                let parent_contour: Vec<(f64, f64)> = path_group
                    .parent_contour
                    .path
                    .iter()
                    .map(|point| (point.x as f64, point.y as f64))
                    .chain(std::iter::once((
                        path_group.parent_contour.path[0].x as f64,
                        path_group.parent_contour.path[0].y as f64,
                    )))
                    .collect();
                let child_contours: Vec<Vec<(f64, f64)>> = path_group
                    .child_contours
                    .iter()
                    .map(|contour| {
                        contour
                            .path
                            .iter()
                            .map(|point| (point.x as f64, point.y as f64))
                            .chain(std::iter::once((
                                contour.path[0].x as f64,
                                contour.path[0].y as f64,
                            )))
                            .collect()
                    })
                    .collect();
                //提取高度参数
                let height_params: Vec<f64> = std::iter::once(path_group.parent_contour.height as f64)
                    .chain(
                        path_group
                            .child_contours
                            .iter()
                            .map(|contour| contour.height as f64),
                    )
                    .collect();

                //生成模型函数，存储到结构体变量中相关模型信息
                model_generate.generate_model(
                    &parent_contour,
                    &parent_contour,
                    &child_contours,
                    &height_params,
                );
            // };

            //保存模型文件到目录中
            model_generate.save_model_file(model_save_path.as_str());

            // let obj_data = std::fs::read_to_string(&model_save_path).unwrap();
            // let model_info = crate::models::ModelInfo {
            //     model_id: 0, // Replace with actual model ID if available
            //     model_data: base64::encode(obj_data),
            // };
            // models.push(model_info);

            let svg_save_path = format!("data/{}/{}/svgs", path_group.user_id, path_group.city_id);
            let svg_file_name = format!(
                "{}/model_{}_{}_{}.svg",
                svg_save_path,
                path_groups.session_id,
                chrono::Utc::now().timestamp(),
                uuid::Uuid::new_v4()
            );
            //保存svg文件到svg和file数据库记录中
            let (svg_id_pk, svg_file_id) = crate::service::FileManager::save_new_svg_database(
                path_group,
                svg_file_name.as_str(),
                pool,
                svg_height.into(),
                svg_width.into(),
            )
                .await?;

            let new_model = crate::models::entity::model::NewModel {
                model_path: model_save_path.clone(),
                city_id: Some(path_group.city_id),
                svg_id: Some(svg_id_pk),
            };
            //保存模型文件到svg和file数据库记录中
            let (model_id_pk, model_file_id) =
                FileManager::save_new_model_database(&new_model, pool).await?;

            //将模型信息存储到结构体变量中，并发送回前端
            let obj_data = std::fs::read_to_string(&model_save_path).unwrap();
            let model_info = crate::models::ModelInfo {
                model_id: model_id_pk, // Replace with actual model ID if available
                model_data: base64::engine::general_purpose::STANDARD.encode(obj_data),
            };
            models.push(model_info);
        }

        //返回模型组信息
        let response = ModelResponse {
            user_id: path_groups.path_groups[0].user_id,
            city_id: path_groups.path_groups[0].city_id,
            models,
        };

        Ok(response)
    }
    //同级模型合并，将同级模型合并成一个模型，因为一个建筑有多个模型，每个模型都是一个独立的模型，合并后针对整体的模型进行元数据存取
    // pub fn merge_models(models: Vec<crate::models::ModelInfo>) -> Result<String, Box<dyn Error>> {
    //     let mut merged_model = crate::service::model_merge::ModelMerge::new();
    //     for model in models {
    //         let obj_data = base64::engine::general_purpose::STANDARD.decode(model.model_data.as_bytes()).unwrap();
    //         let obj_data = String::from_utf8(obj_data).unwrap();
    //         merged_model.add_model(&obj_data);
    //     }
    //     let merged_model_path = format!(
    //         "data/merged_models/model_{}_{}.obj",
    //         chrono::Utc::now().timestamp(),
    //         uuid::Uuid::new_v4()
    //     );
    //     merged_model.save_merged_model(merged_model_path.as_str());
    //
    //     Ok(merged_model_path)
    // }
}

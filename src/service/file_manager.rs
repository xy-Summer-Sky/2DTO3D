// use crate::utils::convert_to_h264_aac;
use crate::dao::ModelModule::ModelDao;
use crate::dao::SvgDao::SvgDao;
use crate::dao::{CityDao, FileDao, ImageDao};
use crate::models::entity::city::NewCity;
use crate::models::entity::file::{File, NewFile};
use crate::models::entity::image::Image;
use crate::models::entity::model::NewModel;
use crate::models::request_models_dto::PathGroup;
use crate::pool::app_state::DbPool;
use chrono::Utc;
use clap::builder::TypedValueParser;
use futures_util::TryFutureExt;
use std::io::Write;
use std::path::Path;
use image::load_from_memory;
use tokio::fs;
use tokio::io::BufWriter;

pub(crate) struct FileManager;

impl FileManager {
    async fn receive_file(
        user_id: &str,
        path: &str,
        size: i64,
        pool: &DbPool,
        file_type: &str,
    ) -> Result<i32, String> {
        // Create a new File instance
        let new_file = NewFile {
            user_id: Some(user_id.parse().map_err(|_| "Invalid user_id")?),
            path: Some(path.to_string()),
            file_type: Some(file_type.to_string()), // You can set the appropriate file type
            size: Some(size),
            created_at: Some(chrono::Utc::now().naive_utc()),
            updated_at: Some(chrono::Utc::now().naive_utc()),
            permissions: Some("default".to_string()), // Set default permissions
        };

        // Save the new_file to the database using FileDao
        let file_id = FileDao::create_file(pool, &new_file)
            .map_err(|e| format!("Failed to insert file: {:?}", e))?;

        Ok(file_id)
    }

    //将用户id、图片id和城市id关联起来，作为返回值
    //保存到目录中。image表中插入数据
    //图片做为文件保存插入到数据库中
    pub(crate) async fn receive_image(
        city_id: i32,
        file_content: &Vec<u8>,
        user_id: i32,
        image_name: &str,
        session_id: &str,
        pool: &DbPool,
    ) -> Result<(i32, i32, i32), String> {
        use image::{load_from_memory, ImageFormat};
        use std::fs::File;
        use std::io::BufWriter;
        use std::path::Path;
      //图片数据原本为Vec<u8>，保存到目录中,需要处理后才可以正常保存
let img = load_from_memory(file_content).map_err(|e| format!("Failed to load image from memory: {:?}", e))?;
        //保存到目录中
        use chrono::Utc;
        let timestamp = Utc::now().timestamp();
        let new_file_name = format!("{}_{}_{}.png", timestamp, session_id, image_name);
        let base_path = format!("data/{}/{}/images/", user_id.to_string(), city_id.to_string());
        let file_path = Path::new(&base_path).join(new_file_name);
        let file = File::create(&file_path).unwrap();
        let ref mut w = BufWriter::new(file);
        img.write_to(w, ImageFormat::Png).unwrap();
        // fs::write(file_path.clone(), img)
        //     .await
        //     .map_err(|e| format!("Failed to save file: {:?}", e))?;

        //将图片记录生成到数据库中
        let new_image = crate::models::entity::image::NewImage {
            image_path: Some(file_path.to_string_lossy().to_string()),
            user_id: Some(user_id),
            city_id: Some(city_id),
        };

        let image_id = crate::dao::ImageDao::create_image_and_get_id(pool, &new_image)
            .await
            .map_err(|e| format!("Failed to insert image: {:?}", e))?;

        //图片做为文件保存插入到数据库中
        Self::receive_file(
            &user_id.to_string(),
            &file_path.to_string_lossy(),
            file_content.len() as i64,
            pool,
            "image",
        )
        .await
        .expect("TODO: panic message");

        // 将用户id、图片id和城市id关联起来，作为返回值
        Ok((user_id, image_id, city_id))
    }

    ///接收原始的轮廓svg并且存储为文件，并且将文件记录插入到数据库中，保存到目录中
    pub async fn receive_svg(
        original_svg: crate::models::request_models_dto::OriginalSvg,
        pool: &DbPool,
    ) -> Result<(i32, i32, i32, i32, String), String> {
        // Save the SVG content to a file
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| format!("Failed to get timestamp: {:?}", e))?
            .as_secs();
        let file_base_path = format!(
            "data/{}/{}/svgs",
            original_svg.user_id, original_svg.city_id
        );
        let file_name = format!(
            "{}/original_svg_{}_{}_{}.svg",
            file_base_path, original_svg.session_id, original_svg.city_id, timestamp
        );
        let mut file = std::fs::File::create(&file_name)
            .map_err(|e| format!("Failed to create file: {:?}", e))?;
        file.write_all(original_svg.svg_content.as_bytes())
            .expect("TODO: panic message");

        // Save the SVG file to the database
        // Save the 文件记录 to the database using FileManager
        let file_id = Self::receive_file(
            &original_svg.user_id.to_string(),
            &file_name,
            file_name.len() as i64,
            pool,
            "svg",
        )
        .await
        .map_err(|_| "Failed to save svg file".to_string())?;

        //保存svg记录到数据库中,使用#[derive(Insertable, Queryable, Serialize, Deserialize)]
        // #[diesel(table_name = svgs)]
        // pub struct NewSvg {
        //     #[diesel(column_name = city_id)]
        //     pub city_id: Option<i32>,
        //     #[diesel(column_name = svg_path)]
        //     pub svg_path: String,
        //     #[diesel(column_name = image_id)]
        //     pub image_id: Option<i32>,
        //     #[diesel(column_name = image_path)]
        //     pub image_path: Option<String>,
        // }
        let image: Image = ImageDao::get_image_by_id(pool, original_svg.image_id).unwrap();
        let new_svg = crate::models::entity::svg::NewSvg {
            city_id: Some(original_svg.city_id),
            svg_path: file_name,
            image_id: Some(original_svg.image_id),
            image_path: image.image_path,
        };
        SvgDao::create_svg(pool, &new_svg).expect("TODO: panic message");

        // Return the user_id, image_id, city_id, file_id, and the original SVG content
        Ok((
            original_svg.user_id,
            original_svg.image_id,
            original_svg.city_id,
            file_id,
            original_svg.svg_content,
        ))
    }

    //构建新的city记录，并创建相关的目录,返回最后插入的city——id
    pub(crate) async fn new_city_and_new_directory(
        pool: &DbPool,
        user_id: &i32,
        city_name: &str,
    ) -> Result<i32, String> {
        let user_id_clone = user_id.to_string();
        let pool_clone = pool.clone();
        let city_name = city_name.to_string();
        let (city_id, city_root_path) = tokio::task::spawn_blocking(move || {
            CityDao::create_city_with_model_path(
                &pool_clone,
                &NewCity {
                    user_id: Some(user_id_clone.parse().map_err(|_| "Invalid user_id")?),
                    city_name: city_name.to_string(),
                },
            )
            .map_err(|_| "Failed to create city")
        })
        .await
        .map_err(|_| "Task join error")??;
        //创建相关的目录 images,svgs,obj_models,city_models
        let base_path = format!("data/{}/{}", user_id, city_id);
        let directories = [
            "images",
            "svgs",
            "obj_models",
            "city_models",
            "click_positions",
            "videos",
        ];
        for dir in &directories {
            let dir_path = Path::new(&base_path).join(dir);
            fs::create_dir_all(&dir_path).await.map_err(|e| {
                format!("Failed to create directory: {:?}, error: {:?}", dir_path, e)
            })?;
        }

        Ok(city_id)
    }

    //存储新的用户分完类的svg到数据库记录中
    pub(crate) async fn save_new_svg_database(
        path_group: &PathGroup,
        file_path_name: &str,
        pool: &DbPool,
        svg_height:f64,
        svg_width:f64
    ) -> Result<(i32, i32), String> {
        //根据image_id获取image记录中的iamge_path
        let image: Image = ImageDao::get_image_by_id(pool, path_group.image_id)
            .map_err(|e| format!("Failed to get image: {:?}", e))?;
        let new_svg = crate::models::entity::svg::NewSvg {
            city_id: Some(path_group.city_id),
            svg_path: file_path_name.to_string(),
            image_id: Some(path_group.image_id),
            image_path: image.image_path,
        };

        //将新的svg记录插入到数据库中
        let svg_id_pk: i32 = SvgDao::create_svg(pool, &new_svg)
            .map_err(|e| format!("Failed to insert svg: {:?}", e))?;

        //插入新的file记录到数据库中
        let file_id = Self::receive_file(
            &path_group.user_id.to_string(),
            &file_path_name,
            file_path_name.len() as i64,
            pool,
            "svg",
        ).await?;

        //保存到实际的文件目录中
        let svg_file = path_group.to_svg(&mut std::fs::File::create(file_path_name).map_err(|e| format!("Failed to create file: {:?}", e))?,svg_height,svg_width).unwrap();

        Ok((svg_id_pk, file_id))
    }

    pub async fn save_new_model_database(
        new_model: &NewModel,
        pool: &DbPool,
    ) -> Result<(i32, i32), String> {
        //插入新的model记录到数据库中
     let model_id_pk: i32 = ModelDao::create_model(pool, new_model)
    .await
    .map_err(|e| format!("Failed to insert model: {:?}", e))?;
        //保存file记录
        let file_id = Self::receive_file(
            &new_model.city_id.unwrap().to_string(),
            &new_model.model_path,
            new_model.model_path.len() as i64,
            pool,
            "model",
        )
        .map_err(|e| format!("Failed to save model file: {:?}", e)).await?;
        Ok((model_id_pk, file_id))
    }
}

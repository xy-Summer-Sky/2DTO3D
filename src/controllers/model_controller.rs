use crate::models::{ModelResponse, SingleModelResponse};
use crate::pool::app_state::AppState;
use crate::service::{CitiesManagement, FileManager, ModelsManagement, SessionData};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use crate::models::request_models_dto::{ExtractContourRequestData, ImageUpload, PathGroups};

pub type RedisPool = Pool<RedisConnectionManager>;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(new_city);
    cfg.service(upload_image);
    cfg.service(extract_contours);
    cfg.service(build_model);
    cfg.service(get_city_models);
    cfg.service(get_model_by_id);
    cfg.service(get_model_ids);
    cfg.service(get_city_ids_by_user_id);
}

/// ### `GET /new_city/{city_name}/{user_id}`
///
/// **Description:**
/// Creates a new city and associates it with a user.
///
/// **Request:**
/// ```http
/// GET /new_city/{city_name}/{user_id} HTTP/1.1
/// Host: example.com
/// ```
///
/// **Response:**
/// ```json
/// {
///     "message": "New city: {city_name}"
/// }
/// ```
///
/// **Response Codes:**
/// - `200 OK`: City created successfully.
/// - `500 Internal Server Error`: Failed to create city.
#[utoipa::path(
    get,
    path = "/model/new_city/{city_name}/{user_id}",
   responses(
    (status = 200, description = "City created successfully", body = (i32, String), example = json!({"message": "New city: test_city", "city_id": 1})),
    (status = 500, description = "Internal server error", body = String, example = json!("Failed to create city"))
),
    params(
        ("city_name" = String, Path, description = "City name"),
        ("user_id" = i32, Path, description = "User ID")
    )
)]
#[get("/new_city/{city_name}/{user_id}")]
pub async fn new_city(
    app_state: web::Data<AppState>,
    path: web::Path<(String, i32)>,
) -> impl Responder {
    let pool = &app_state.pool;
    let redis_pool = &app_state.redis_pool;
    let (city_name, user_id) = path.into_inner();
    match FileManager::new_city_and_new_directory(&pool, &user_id, &city_name).await {
        Ok(city_id) => HttpResponse::Ok().body(format!("New city: {}, city_id: {}", city_name, city_id)),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to create city: {}", e)),
    }
}

/// ### `POST /upload_image`
///
/// **Description:**
/// Uploads an image and associates it with a user and city.
///
/// **Request:**
/// ```http
/// POST /upload_image HTTP/1.1
/// Host: example.com
/// Content-Type: multipart/form-data
/// Cookie: session_id=your_session_id
///
/// --boundary
/// Content-Disposition: form-data; name="image"; filename="example.png"
/// Content-Type: image/png
///
/// <binary image data>
/// --boundary--
/// ```
///
/// **Response:**
/// ```json
/// {
///     "user_id": 123,
///     "image_id": 456,
///     "city_id": 789
/// }
/// ```
///
/// **Response Codes:**
/// - `200 OK`: Image uploaded successfully.
/// - `400 Bad Request`: Missing or invalid parameters.
/// - `500 Internal Server Error`: Failed to upload image.
#[utoipa::path(
    post,
    path = "/model/upload_image",
    responses(
        (status = 200, description = "Image uploaded successfully", body = (i32, i32, i32), example = json!({"user_id": 123, "image_id": 456, "city_id": 789})),
        (status = 500, description = "Internal server error")
    ),
    request_body(
        content = ImageUpload,
        description = "Image upload data"
    )
)]
#[post("/upload_image")]
pub async fn upload_image(
    image_upload: crate::models::request_models_dto::ImageUpload,
    payload: web::Payload,
    app_state: web::Data<AppState>,
) -> impl Responder {
    //获取redis连接
    let mut redis_conn = match app_state.redis_pool.get().await {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to get Redis connection: {}", e))
        }
    };
    //获取session——id
    let session_id = match image_upload.cookie.clone() {
        Some(cookie) => cookie,
        None => return HttpResponse::BadRequest().body("Missing session_id cookie"),
    };
    //利用redis解析session_id,获取session_data,目前不获取任何信息
    let session_data = SessionData::get_session_data_by_id(&mut *redis_conn, &session_id).await;

    //保存了图片到目录和数据库表中记录
    //保存上传的图片，将用户id、图片id和城市id关联起来，作为返回值
    //实现了image、file表中数据的插入
    let city_id = image_upload.user_info.city_id;
    let user_id = image_upload.user_info.user_id;

    // let mut payload = image_upload.image_data;
    let file_content = crate::models::request_models_dto::upload_image_process_image(payload).await;

    match FileManager::receive_image(
        city_id,
        &file_content,
        user_id,
        &image_upload.image_name,
        &session_id,
        &app_state.pool,
    )
    .await
    {
        Ok((user_id, image_id, city_id)) => HttpResponse::Ok().json((user_id, image_id, city_id)),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to upload image: {}", e))
        }
    }
    //返回值是一个元组，包含了用户id、图片id和城市id
    //轮廓提取在其它模块中实现
}

/// ### `POST /extract_contours`
///
/// **Description:**
/// Extracts contours from an uploaded image.
///
/// **Request:**
/// ```http
/// POST /extract_contours HTTP/1.1
/// Host: example.com
/// Content-Type: application/json
/// Cookie: session_id=your_session_id
///
/// {
///     "user_id": 123,
///     "city_id": 456,
///     "image_id": 789,
///     "right_clicks": [
///         {
///             "x": 100,
///             "y": 150,
///             "type": "right"
///         },
///         {
///             "x": 200,
///             "y": 250,
///             "type": "left"
///         }
///     ],
///     "image_data": "QWERTYUIOPASDFGHJKLZXCVBNM=="
/// }
/// ```
///
/// **Response:**
/// ```json
/// {
///     "user_id": 123,
///     "image_id": 456,
///     "city_id": 789,
///     "file_id": 101112,
///     "svg_content": "<svg>...</svg>"
/// }
/// ```
///
/// **Response Codes:**
/// - `200 OK`: Contours extracted successfully.
/// - `400 Bad Request`: Missing or invalid parameters.
/// - `500 Internal Server Error`: Failed to extract contours.
#[utoipa::path(
    post,
    path = "/model/extract_contours",
    responses(
        (status = 200, description = "Contours extracted successfully", body = (i32, i32, i32, i32, String), example = json!({"user_id": 123, "image_id": 456, "city_id": 789, "file_id": 101112, "svg_content": "<svg>...</svg>"})),
        (status = 500, description = "Internal server error")
    ),
    request_body(
        content = ExtractContourRequestData,
        description = "Data for extracting contours"
    )
)]
#[post("/extract_contours")]
pub async fn extract_contours(
    req: HttpRequest,
    app_state: web::Data<AppState>,
    image_upload: crate::models::request_models_dto::ExtractContourRequestData,
) -> impl Responder {
    let pool = &app_state.pool;
    let (svg, svg_with, svg_height) =
        match crate::service::ModelApiIntegrate::extract_contours(&image_upload, &pool).await {
            Ok(svg) => svg,
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Failed to extract contours: {}", e));
            }
        };

    let session_id = req
        .cookie("session_id")
        .map(|c| c.value().to_string())
        .unwrap_or_else(|| "default_session_id".to_string());
    //svg是提取的轮廓，此处进行保存（存储到目录中并且存储到数据库记录中）
    let original_svg = crate::models::request_models_dto::OriginalSvg {
        user_id: image_upload.user_id,
        city_id: image_upload.city_id,
        image_id: image_upload.image_id,
        session_id: session_id,
        svg_content: svg.clone(),
    };
    //保存到目录中,同时也保存到数据库记录中
    // Return the user_id, image_id, city_id, file_id, and the original SVG content
    let response = FileManager::receive_svg(original_svg, pool).await.unwrap();

    //返回值是一个元组，包含了用户id、图片id和城市id和svg内容
    HttpResponse::Ok().json(response)
}

//用户在众多轮廓中进行分组，每一组都有一个父轮廓和许多子轮廓，每一个轮廓都有着不同的高度，提交这些信息进行模型构建
//原本的svg中，每一个轮廓都会有一个id，用户只需要选择不同的轮廓id，就可以进行分组，标记出每一组轮廓的父轮廓和子轮廓，并且赋予每个轮廓高度参数
/// ### `POST /build_model`
///
/// **Description:**
/// Builds a model based on the provided path groups.
///
/// **Request:**
/// ```http
/// POST /build_model HTTP/1.1
/// Host: example.com
/// Content-Type: application/json
/// Cookie: session_id=your_session_id
///
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
/// **Response:**
/// ```json
/// {
///     "user_id": 123,
///     "city_id": 456,
///     "models": [
///         {
///             "model_id": 789,
///             "model_data": "base64_encoded_obj_data"
///         }
///     ]
/// }
/// ```
#[utoipa::path(
    post,
    path = "/model/build_model",
    responses(
        (status = 200, description = "Model built successfully", body = ModelResponse),
        (status = 500, description = "Internal server error")
    ),
    request_body(
        content = PathGroups,
        description = "Path groups for model building"
    )
)]
#[post("/build_model")]
pub(crate) async fn build_model(
    req: HttpRequest,
    path_groups: web::Json<crate::models::request_models_dto::PathGroups>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    let pool = &app_state.pool;

    // Perform model building logic here
    let models =
        match crate::service::ModelApiIntegrate::generate_from_path_groups(&path_groups, &pool)
            .await
        {
            Ok(model_response) => model_response,
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Failed to build model: {}", e))
            }
        };
    //返回模型文件用于解码渲染
    HttpResponse::Ok().json(models)
}


#[utoipa::path(
    get,
    path = "/model/get_city_models/{city_id}",
    responses(
       (status = 200, description = "City models retrieved successfully", body = [ (i32, String) ], example = json!([(1, "obj file data here")])),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("city_id" = i32, Path, description = "City ID")
    )
)]
#[get("/get_city_models/{city_id}")]
pub async fn get_city_models(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let pool = &app_state.pool;
    let city_id = path.into_inner();
    match ModelsManagement::get_city_models_by_city_id(&pool, city_id).await {
        Ok(models) => HttpResponse::Ok().json(models),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to get city models: {}", e))
        }
    }
}

#[utoipa::path(
    get,
    path = "/model/get_model_by_id/{model_id}",
    responses(
        (status = 200, description = "Model retrieved successfully",body=SingleModelResponse),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("model_id" = i32, Path, description = "Model ID")
    )
)]
#[get("/get_model_by_id/{model_id}")]
pub async fn get_model_by_id(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let pool = &app_state.pool;
    let model_id = path.into_inner();
    match ModelsManagement::get_model_by_id(&pool, model_id).await {
        Ok(model) => {
            let model_response = SingleModelResponse::new(model.0, model.1);
            HttpResponse::Ok().json(model_response)
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to get model by id: {}", e))
        }
    }
}

#[utoipa::path(
    get,
    path = "/model/get_model_ids/{city_id}",
    responses(
       (status = 200, description = "Model IDs retrieved successfully", body = [i32], example = json!([1, 2, 3])),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("city_id" = i32, Path, description = "City ID")
    )
)]
#[get("/get_model_ids/{city_id}")]
pub async fn get_model_ids(app_state: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let pool = &app_state.pool;
    let city_id = path.into_inner();
    match ModelsManagement::get_model_ids_by_city_id(&pool, city_id).await {
        Ok(models) => HttpResponse::Ok().json(models),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to get city models: {}", e))
        }
    }
}


#[utoipa::path(
    get,
    path = "/model/get_city_ids_by_user_id/{user_id}",
    responses(
        (status = 200, description = "City IDs retrieved successfully",body=[i32]),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("user_id" = i32, Path, description = "User ID")
    )
)]
#[get("/get_city_ids_by_user_id/{model_id}")]
pub async fn get_city_ids_by_user_id(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let pool = &app_state.pool;
    let user_id = path.into_inner();
    match CitiesManagement::get_city_ids_by_user_id(&pool, user_id) {
        Ok(models) => HttpResponse::Ok().json(models),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to get city models: {}", e))
        }
    }
}

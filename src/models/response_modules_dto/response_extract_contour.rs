use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    lon: f64,
    lat: f64,
    height: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Contour {
    contour_points: Vec<Point>,
    height: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ContourPoints {
    parent: Contour,
    children: Vec<Contour>,
}



/// Example response format for `ExtractContourResponseData`:
///
/// ```http
/// HTTP/1.1 200 OK
/// Content-Type: application/json
///
/// {
///   "user_id": 102,
///    "city_id": 301,
///     "image_id": 50321,
///     "contour_points": [
///         {
///             "parent": {
///                 "contour_points": [
///                     {"lon": 120.1234, "lat": 35.1234, "height": 10},
///                     {"lon": 120.1256, "lat": 35.1256, "height": 10},
///                     {"lon": 120.1278, "lat": 35.1278, "height": 10}
///                 ],
///                 "height": 10
///             },
///             "children": [
///                 {
///                     "contour_points": [
///                         {"lon": 120.1280, "lat": 35.1280, "height": 5},
///
///                     {"lon": 120.1290, "lat": 35.1290, "height": 5},
///
///                     {"lon": 120.1300, "lat": 35.1300, "height": 5}
///                   ],
///                    "height": 5
///                }
///             ]
///         },
///         {
///             "parent": {
///                 "contour_points": [
///                     {"lon": 120.1334, "lat": 35.1334, "height": 15},
///                     {"lon": 120.1356, "lat": 35.1356, "height": 15},
///                     {"lon": 120.1378, "lat": 35.1378, "height": 15}
///                 ],
///                 "height": 15
///             },
///             "children": [
///                 {
///                     "contour_points": [
///                         {"lon": 120.1380, "lat": 35.1380, "height": 8},
///                         {"lon": 120.1390, "lat": 35.1390, "height": 8},
///                         {"lon": 120.1400, "lat": 35.1400, "height": 8}
///                     ],
///                     "height": 8
///                 },
///                 {
///                     "contour_points": [
///                         {"lon": 120.1402, "lat": 35.1402, "height": 4},
///                         {"lon": 120.1412, "lat": 35.1412, "height": 4},
///                         {"lon": 120.1422, "lat": 35.1422, "height": 4}
///                     ],
///                     "height": 4
///                 }
///             ]
///         }
///     ]
/// }
///
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct ExtractContourResponseData {
    user_id: i32,
    city_id: i32,
    image_id: i32,
    contour_points: Vec<ContourPoints>,
}

/// Example response format for `Contours`:
///
/// ```http
/// HTTP/1.1 200 OK
/// Content-Type: application/json
///
/// {
///    "user_id": 102,
///   "city_id": 301,
///   "image_id": 50321,
///  "contours": [
///     {
///        "contour_points": [
///          {"lon": 120.1234, "lat": 35.1234, "height": 10},
///         {"lon": 120.1256, "lat": 35.1256, "height": 10},
///        {"lon": 120.1278, "lat": 35.1278, "height": 10}
///    ],
///   "height": 10
/// },
/// {
///   "contour_points": [
///    {"lon": 120.1280, "lat": 35.1280, "height": 5},
///   {"lon": 120.1290, "lat": 35.1290, "height": 5},
/// {"lon": 120.1300, "lat": 35.1300, "height": 5}
/// ],
/// "height": 5
/// }
///
///     ]
/// }
/// ```
#[derive(Serialize, Deserialize, Debug)]
struct Contours {
    user_id: i32,
    city_id: i32,
    image_id: i32,
    contours: Vec<Contour>,
}


/// Example response format for `ModelResponse`:
#[derive(Serialize, Deserialize, Debug,ToSchema)]
pub struct ModelResponse {
    pub(crate) user_id: i32,
    pub(crate) city_id: i32,
    pub(crate) models: Vec<ModelInfo>,
}



#[derive(Serialize, Deserialize, Debug,ToSchema)]
pub struct ModelInfo {
    pub model_id: i32,
    pub model_data: String, // Base64 encoded .obj data
}
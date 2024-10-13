#[cfg(test)]
mod model_controller_tests {
    use actix_web::{test, web, App};
    use photosprocess::config::routes::config_user_routes;
    use photosprocess::pool::app_state::AppState;

    #[actix_rt::test]
    async fn test_new_city() {
        let app_state = AppState::new().await;
        let _ = env_logger::builder().is_test(true).try_init();
        let mut app = test::init_service(
            App::new()
                .app_data(actix_web::web::Data::new(app_state.clone()))
                .configure(config_user_routes),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/model/new_city/test_city/1")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        println!("Response: {:?}", &resp);
        println!("Response Body: {:?}", &resp.response().body());

        if let Some(err) = resp.response().error() {
            println!("Error: {:?}", err);
        }

        assert!(resp.status().is_success());
    }
    #[actix_rt::test]
    async fn test_upload_image() {
        //先将指定图片转换为Vec<u8>格式
        let image_data = include_bytes!("../assets/images/contours/test2.svg.png");
        let app_state = AppState::new().await;
        let _ = env_logger::builder().is_test(true).try_init();
        let mut app = test::init_service(
            App::new()
                .app_data(actix_web::web::Data::new(app_state.clone()))
                .configure(config_user_routes),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/model/upload_image?user_id=1&city_id=1&image_name=example_image.png")
            .insert_header(("Content-Type", "multipart/form-data"))
            .insert_header(("Cookie", "session_id=test_session_id"))
            .set_payload(image_data.as_ref())
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        println!("{:?}", &resp);
        println!("{:?}", &resp.response().body());
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_extract_contours() {
        let image_data = include_bytes!("../assets/images/img_1.png");
        let app_state = AppState::new().await;
        let _ = env_logger::builder().is_test(true).try_init();
        let mut app = test::init_service(
            App::new()
                .app_data(actix_web::web::Data::new(app_state.clone()))
                .configure(config_user_routes),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/model/extract_contours")
            .insert_header(("Content-Type", "application/json"))
            .insert_header(("Cookie", "session_id=test_session_id"))
            .set_payload(format!(
                r#"{{
"user_id": 1,
"city_id": 1,
"image_id": 6,
"right_clicks": [
{{
"x": 186,
"y": 179,
"type": "right"
}},
{{
"x": 270,
"y": 242,
"type": "right"
}},
{{
"x": 606,
"y": 215,
"type": "right"
}},
{{
"x": 655,
"y": 123,
"type": "right"
}},
{{
"x": 626,
"y": 634,
"type": "right"
}},
{{
"x": 759,
"y": 635,
"type": "right"
}},
{{
"x": 683,
"y": 802,
"type": "right"
}},
{{
"x": 334,
"y": 998,
"type": "right"
}},
{{
"x": 313,
"y": 866,
"type": "right"
}},
{{
"x": 257,
"y": 586,
"type": "right"
}},
{{
"x": 354,
"y": 523,
"type": "right"
}}
],
"image_data": "{}"
}}"#,
                base64::encode(&image_data)
            ))
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        println!("{:?}", &resp);
        println!("{:?}", &resp.response().body());
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_build_model() {
        let app_state = AppState::new().await;
        let _ = env_logger::builder().is_test(true).try_init();
        let mut app = test::init_service(
            App::new()
                .app_data(actix_web::web::Data::new(app_state.clone()))
                .configure(config_user_routes),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/model/build_model")
            .insert_header(("Content-Type", "application/json"))
            .insert_header(("Cookie", "session_id=test_session_id"))
            .set_payload(
                r#"{
                "session_id": "test_session_id",
           "path_groups": [
            {
                            "city_id": 1,
                            "user_id": 1,
                            "parent_contour": {
                                "path": [
                                 { "x": 2, "y": 2 },
{ "x": 195, "y": 0 },
{ "x": 0, "y": -190 },
{ "x": -195, "y": 0 },
{ "x": 0, "y": 190 }
                                ],
                                "height": 50
                            },
                            "child_contours": [
                                {
                                    "path": [
{ "x": 134, "y": 17 },
{ "x": -20, "y": -4 },
{ "x": -22, "y": 0 },
{ "x": -9, "y": 2 },
{ "x": -9, "y": 4 },
{ "x": -14, "y": 14 },
{ "x": -7, "y": 20 },
{ "x": 0, "y": 108 },
{ "x": 99, "y": 0 },
{ "x": 0, "y": -138 },
{ "x": -18, "y": -6 }
                                    ],
                                    "height": 76
                                }
                            ],
                            "image_id": 6
                        },
    {
        "city_id": 1,
        "user_id": 1,
        "parent_contour": {
            "path": [
                {"x": -20.0, "y": -20.0},
                {"x": -2.0, "y": 192.0},
                {"x": 197.0, "y": 192.0},
                {"x": 197.0, "y": -50.0}
            ],
            "height": 50.0
        },
        "child_contours": [
            {
                "path": [
                    {"x": 134.0, "y": 17.0},
                    {"x": 114.0, "y": 13.0},
                    {"x": 92.0, "y": 13.0},
                    {"x": 83.0, "y": 15.0},
                    {"x": 74.0, "y": 19.0},
                    {"x": 60.0, "y": 33.0},
                    {"x": 53.0, "y": 53.0},
                    {"x": 53.0, "y": 161.0},
                    {"x": 152.0, "y": 161.0},
                    {"x": 152.0, "y": 23.0}
                ],
                "height": 100.0
            }
        ],
        "image_id": 6
    },
    {
        "city_id": 1,
        "user_id": 1,
        "parent_contour": {
            "path": [
                {"x": 10.0, "y": 10.0},
                {"x": 10.0, "y": 200.0},
                {"x": 200.0, "y": 200.0},
                {"x": 200.0, "y": 10.0}
            ],
            "height": 50.0
        },
        "child_contours": [
        ],
        "image_id": 6
    }
]
            }"#,
            )
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        println!("{:?}", &resp);
        println!("{:?}", &resp.response().body());
        assert!(resp.status().is_success());
    }


    use photosprocess::controllers::model_controller::{get_city_models, get_model_by_id};

    #[actix_rt::test]
    async fn test_get_city_models() {

        let app_state = AppState::new().await;
        let _ = env_logger::builder().is_test(true).try_init();
        let mut app = test::init_service(
            App::new()
                .app_data(actix_web::web::Data::new(app_state.clone()))
                .configure(config_user_routes),
        ).await;
        let req = test::TestRequest::get().uri("/model/get_city_models/1").to_request();
        let resp = test::call_service(&mut app, req).await;
        println!("{:?}", &resp);
        println!("{:?}", &resp.response().body());
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_get_model_by_id() {
        let app_state = AppState::new().await;
        let _ = env_logger::builder().is_test(true).try_init();
        let mut app = test::init_service(
            App::new()
                .app_data(actix_web::web::Data::new(app_state.clone()))
                .configure(config_user_routes),
        ).await;
        let req = test::TestRequest::get().uri("/model/get_model_by_id/886").to_request();
        let resp = test::call_service(&mut app, req).await;
        println!("{:?}", &resp);
        println!("{:?}", &resp.response().body());
        assert!(resp.status().is_success());
    }
}

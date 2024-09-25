// tests/user_controller_tests.rs
use actix_web::{test, web, App};
use photosprocess::config::routes::config_user_routes;
// 根据实际模块路径调整
use photosprocess::pool::app_state::DbPool;
use rand::Rng;

#[actix_rt::test]
async fn test_verify_user_route() {
    let pool = setup_test_db_pool().await; // 假设您有一��设置测试数据库连接池的函数
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .configure(config_user_routes)
    ).await;

    println!("Test /user/verify route");
    // 测试 /user/verify 路由

    let identifier: String = rand::thread_rng().gen_range(1000..9999).to_string();
    let password: String = rand::thread_rng().gen_range(1000..9999).to_string();

    let req = test::TestRequest::get()
        .uri(&format!("/user/verify/{}/{}", identifier, password))
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 404); // 假设用户不存在
}
#[actix_rt::test]
async fn test_register_user_route() {
    let pool = setup_test_db_pool().await; // 假设您有一个设置测试数据库连接池的函数
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .configure(config_user_routes)
    ).await;

    println!("Test /user/register route");
    // 测试 /user/register 路由

    let username = rand::thread_rng().gen_range(1000..9999).to_string(); // 这里可以使用随机生成的用户名
    let password = rand::thread_rng().gen_range(1000..9999).to_string(); // 这里可以使用随机生成的密码
    let req = test::TestRequest::post()
        .uri(&format!("/user/register/{}/{}", username, password))
        .to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), 200); // 假设用户注册成功
}

#[actix_rt::test]
async fn hello_test() {
    let pool = setup_test_db_pool().await;
    let mut app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool))
            .configure(config_user_routes)
    ).await;
    let name = "test";
    let req = test::TestRequest::get().uri(&format!("/user/hello/{}",name)).to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), 200);
}




async fn setup_test_db_pool() -> DbPool {
    photosprocess::pool::app_state::establish_connection()
}


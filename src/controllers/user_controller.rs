
use actix_web::{web, Error, Responder, HttpResponse, get, post};
use actix_web::web::Path;
use diesel::RunQueryDsl;
use crate::dao::UserDao;
use crate::entity::user::NewUser;
use crate::pool::app_state::DbPool;
use crate::schema::users;
use crate::service::UserService;


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(verify_user);
    cfg.service(register_user);
    cfg.service(hello);
}




/// Verifies if a user exists.
///
/// # Arguments
///
/// * `pool` - Database connection pool.
/// * `identifier` - User identifier (e.g., email or username).
/// * `password` - User password.
///
/// # Returns
///
/// * `HttpResponse` - Response indicating whether the user exists.
#[get("/verify/{identifier}/{password}")]
pub async fn verify_user(
    pool: web::Data<DbPool>,
    path: Path<(String, String)>,
) -> Result<HttpResponse, Error> {
    let (identifier, password) = path.into_inner();
    println!("identifier: {}", identifier);
    let user_exists = UserService::verify_user(&pool, &identifier, &password).await;
    println!("uerService verify_user5");
    println!("user_exists: {}", user_exists);
    if user_exists {
        println!("uerService verify_user6");
        Ok(HttpResponse::Ok().body("User exists"))
    } else {
        println!("uerService verify_user7");
        Ok(HttpResponse::NotFound().body("User does not exist"))
    }
}

/// Registers a new user.
///
/// # Arguments
///
/// * `pool` - Database connection pool.
/// * `username` - New user's username.
/// * `password` - New user's password.
///
/// # Returns
///
/// * `HttpResponse` - Response indicating the result of the registration.
///
#[post("/register/{username}/{password}")]
pub async fn register_user(
    pool: web::Data<DbPool>,
    path: Path<(String, String)>,
) -> Result<HttpResponse, Error> {
    let (username, password) = path.into_inner();
    let register_result = UserService::register(&pool, &username, &password).await;
    match register_result {
        Ok(_) => Ok(HttpResponse::Ok().body("User registered successfully")),
        Err(_) => Ok(HttpResponse::InternalServerError().body("Failed to register user")),
    }
}

//测试用，简单api
#[get("/hello/{name}")]
pub async fn hello(db_pool: web::Data<DbPool>,path:Path<String>) -> impl Responder {
    let name = path.into_inner();
    println!("name: {}", name);
    let mut conn= db_pool.get().expect("couldn't get db connection from pool");
  // 假设您有一个设置测试数据库连接池的函数

        // tokio::task::spawn_blocking(move || {
        // diesel::sql_query("INSERT INTO test (string) VALUES ('example_string')")
        //     .execute(&mut conn)
        //     .expect("Failed to insert test data");
        // }).await.expect("Task panicked");

    let user =NewUser {
        username: "name".to_string(),
        password_hash: bcrypt::hash("password", bcrypt::DEFAULT_COST).unwrap(),};
    //插入新用户
    UserDao::create_user(&db_pool, &user).expect("TODO: panic message");

    UserService::register(&db_pool, &user.username, &user.password_hash).await.expect("TODO: panic message");
    // tokio::task::spawn_blocking(move || {
    //     diesel::insert_into(users::table)
    //         .values(user)
    //         .execute(&mut conn)
    //
    // }) .await.expect("Task panicked");

                                HttpResponse::Ok().body("Hello world!")
}
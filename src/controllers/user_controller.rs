use crate::dao::UserDao;
use crate::models::entity::user::NewUser;
use crate::pool::app_state::{AppState, DbPool};
use crate::service::{SessionData, UserService};
use actix_web::web::Path;
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse, Responder};

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
    http_request: HttpRequest,
    app_state: web::Data<AppState>,
    path: Path<(String, String)>,
) -> Result<HttpResponse, Error> {
    let pool = &app_state.pool;
    let redis_pool = &app_state.redis_pool;
    let (identifier, password) = path.into_inner();

    let sessionid = SessionData::get_session_data(http_request, redis_pool)
        .await
        .expect("TODO: panic message");

    let user_exists = UserService::verify_user(pool, &identifier, &password).await;
    if user_exists {
        Ok(HttpResponse::Ok()
            .cookie(
                actix_web::cookie::Cookie::build("session_id", sessionid)
                    .path("/")
                    .http_only(true)
                    .finish(),
            )
            .body("User exists"))
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
        Ok(user_id) => {
            Ok(HttpResponse::Ok()
                .body(format!("User registered successfully with ID: {}", user_id)))
        }
        Err(_) => Ok(HttpResponse::InternalServerError().body("Failed to register user")),
    }
}

//测试用，简单api
#[get("/hello/{name}")]
pub async fn hello(app_state: web::Data<AppState>, path: Path<String>) -> impl Responder {
    let db_pool = &app_state.pool;
    let name = path.into_inner();
    println!("name: {}", name);
    let mut conn = db_pool.get().expect("couldn't get db connection from pool");
    // 假设您有一个设置测试数据库连接池的函数

    // tokio::task::spawn_blocking(move || {
    // diesel::sql_query("INSERT INTO test (string) VALUES ('example_string')")
    //     .execute(&mut conn)
    //     .expect("Failed to insert test data");
    // }).await.expect("Task panicked");

    let user = NewUser {
        username: "name".to_string(),
        password_hash: bcrypt::hash("password", bcrypt::DEFAULT_COST).unwrap(),
    };
    //插入新用户
    UserDao::create_user(&db_pool, &user).expect("TODO: panic message");

    UserService::register(&db_pool, &user.username, &user.password_hash)
        .await
        .expect("TODO: panic message");

    HttpResponse::Ok().body("Hello world!")
}

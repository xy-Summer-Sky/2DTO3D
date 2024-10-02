use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use std::env;
use diesel::r2d2::Pool;

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;
pub type RedisPool = bb8::Pool<bb8_redis::RedisConnectionManager>;
#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<ConnectionManager<MysqlConnection>>,
    pub redis_pool: RedisPool,
}
impl AppState {
    pub async fn new() -> AppState {
        AppState {
            pool: establish_connection(),
            redis_pool: establish_redis_pool().await
        }
    }

}
pub fn establish_connection() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "mysql://members:Helloworld66##@47.84.72.144:3306/3dRender".to_string());
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    println!("数据库连接成功");
    Pool::builder()
        .max_size(20)
        .min_idle(Some(5))
        .build(manager)
        .expect("Failed to create pool.")
}


pub async fn establish_redis_pool() -> bb8::Pool<bb8_redis::RedisConnectionManager> {
    dotenv().ok();
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    let manager = bb8_redis::RedisConnectionManager::new(redis_url).expect("Failed to create Redis connection manager.");
    bb8::Pool::builder()
        .max_size(20)
        .min_idle(Some(5))
        .build(manager)
        .await
        .expect("Failed to create Redis pool.")
}
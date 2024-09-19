use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use std::env;
use diesel::r2d2::Pool;
pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<ConnectionManager<MysqlConnection>> // 数据库连接池
    //线程池子
}
impl AppState {
    pub fn new() -> AppState {
        AppState {
            pool: establish_connection()
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
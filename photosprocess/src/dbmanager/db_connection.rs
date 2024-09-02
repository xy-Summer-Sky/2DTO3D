use sqlx::mysql::MySqlPool;

pub async fn establish_connection() -> MySqlPool {
    MySqlPool::connect("mysql://root:helloworldd@localhost/3d_city").await.unwrap()
}
use sqlx::mysql::MySqlPool;

pub async fn establish_connection() -> MySqlPool {
    MySqlPool::connect("mysql://members:Helloworld66##@47.84.72.144:3306/3dRender").await.unwrap()
}
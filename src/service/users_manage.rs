use crate::dao::UserDao;
use crate::pool::app_state::DbPool;

pub struct UserService;

impl UserService {
    pub async fn verify_user(pool: &DbPool, identifier: &str, password: &str) -> bool {
        let pool = pool.clone();

        let identifier = identifier.to_string().clone();
        let password = password.to_string().clone();
        tokio::task::spawn_blocking(move || UserDao::verify_user(&pool, &identifier, &password))
            .await
            .expect("Task panicked")
    }

    //注册，并且实现注册后创建用户目录，以及用户目录
    pub async fn register(pool: &DbPool, username: &str, password: &str) -> Result<i32, String> {
        let pool = pool.clone();
        let username = username.to_string();
        let password = password.to_string();
        println!("register");
        tokio::task::spawn_blocking(move || {
            let user_id=UserDao::register(&pool, &username, &password)?;
            UserDao::after_register_create_directory(&pool, user_id)
                .map_err(|e| e.to_string())?;
            Ok(user_id)
        })
        .await
        .map_err(|e| e.to_string())?
    }
}

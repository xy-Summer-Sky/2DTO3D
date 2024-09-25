use crate::dao::UserDao;
use crate::pool::app_state::DbPool;

pub struct UserService;

impl UserService {

   pub async fn verify_user(session:actix_session::Session,pool: &DbPool, identifier: &str, password: &str) -> bool {
        let pool = pool.clone();
        let identifier = identifier.to_string().clone();
        let password = password.to_string().clone();
        tokio::task::spawn_blocking(move || {


            UserDao::verify_user(session:actix_session::Session,&pool,&identifier, &password)

        })
            .await
            .expect("Task panicked")

    }


    pub async fn register(pool: &DbPool, username: &str, password: &str) -> Result<(), String> {
        let pool = pool.clone();
        let username = username.to_string();
        let password = password.to_string();
        println!("register");
        tokio::task::spawn_blocking(move || {
            UserDao::register(&pool, &username, &password)?;
            UserDao::after_register_create_directory(&pool, &username).map_err(|e| e.to_string())?;
            Ok(())
        })
            .await
            .map_err(|e| e.to_string())?
    }





}
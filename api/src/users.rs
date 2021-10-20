use crate::{db, db::DbError};
use sqlx::{Pool, Postgres};
use trillium::Conn;
use trillium_api::ApiConnExt;
use types::users::UsersRequest;

pub async fn post_users(mut conn: Conn) -> Conn {
    match conn.deserialize::<UsersRequest>().await {
        Ok(user) => {
            let pool = conn.state::<Pool<Postgres>>().unwrap();

            match db::users::create_user(pool, user).await {
                Ok(user) => conn.with_status(201).with_json(&user),
                Err(e) => match e {
                    DbError::Conflict(_) => conn.with_status(409),
                    _ => {
                        log::error!("POST /users error: {:?}", e);
                        conn.with_status(500)
                    }
                },
            }
        }
        Err(_) => conn.with_status(400),
    }
}

pub async fn login(mut conn: Conn) -> Conn {
    match conn.deserialize::<UsersRequest>().await {
        Ok(user) => {
            let pool = conn.state::<Pool<Postgres>>().unwrap();

            match db::users::get_user_by_username_and_password(pool, user).await {
                Ok(user) => conn.with_status(201).with_json(&user.key),
                Err(e) => match &e {
                    DbError::PasswordNotCorrect => conn.with_status(403),
                    _ => {
                        log::info!("POST /login error: {:?}", e);
                        conn.with_status(500)
                    }
                },
            }
        }
        Err(_) => conn.with_status(400),
    }
}

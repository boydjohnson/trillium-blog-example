use crate::{db, db::DbError};
use sqlx::{Pool, Postgres};
use trillium::Conn;
use trillium_api::ApiConnExt;
use types::users::UsersRequest;

pub async fn post_users(mut conn: Conn) -> Conn {

    match conn.deserialize().await {
        Ok(UsersRequest { username, password }) => {
            let pool = conn.state::<Pool<Postgres>>().unwrap();
            
            match db::create_user(pool, UsersRequest { username, password }).await {
                Ok(user) => conn.with_status(201).with_json(&user),
                Err(e) => match e {
                    DbError::Conflict(_) => conn.with_status(409),
                    _ => {
                        log::error!("POST Users error: {:?}", e);
                        conn.with_status(500)
                    }
                },
            }
        }
        Err(_) => conn.with_status(400),
    }
}

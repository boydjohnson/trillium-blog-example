use crate::{db, db::DbError};
use sqlx::{Pool, Postgres};
use trillium::{Conn, Status};
use trillium_api::ApiConnExt;
use types::users::{User, UsersRequest};

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

pub async fn get_users(conn: Conn) -> Conn {
    if conn.state::<User>().is_some() {
        if let Some(pool) = conn.state::<Pool<Postgres>>() {
            match db::users::list_users(pool).await {
                Ok(res) => conn.with_status(Status::Ok).with_json(&res),
                Err(e) => {
                    log::warn!("GET /users returned a db error: {:?}", e);
                    conn.with_status(Status::InternalServerError)
                }
            }
        } else {
            log::warn!("GET /users expected pool state.");

            conn.with_status(Status::InternalServerError)
        }
    } else {
        conn
    }
}

pub async fn get_users_self(conn: Conn) -> Conn {
    if let Some(user) = conn.state::<User>().cloned() {
        conn.with_status(Status::Ok).with_json(&user)
    } else {
        conn
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use trillium_testing::prelude::*;

    #[test]
    fn test_get_users() {
        dotenv::dotenv().ok();
        env_logger::init();

        assert_response!(
            get("/")
                .with_request_header("Authorization", "Bearer 123")
                .on(&(crate::json_web_token::user_handler, get_users)),
            Status::Forbidden
        );
    }
}

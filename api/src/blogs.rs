use crate::db::{self};
use sqlx::{Pool, Postgres};
use trillium::{Conn, Status};
use trillium_api::ApiConnExt;
use types::{
    blogs::{BlogResponse, NewBlogRequest},
    users::User,
};

pub async fn post_blogs_pre(mut conn: Conn) -> Conn {
    match conn.deserialize::<NewBlogRequest>().await {
        Ok(new_blog) => {
            if let Some(pool) = conn.state::<Pool<Postgres>>() {
                match db::blogs::create_blog(pool, &new_blog).await {
                    Ok(resp) => conn.with_state(resp),
                    Err(e) => {
                        log::info!("POST /blogs error from database: {:?}", e);
                        conn.with_status(Status::InternalServerError)
                    }
                }
            } else {
                conn.with_status(Status::InternalServerError)
            }
        }
        Err(e) => {
            log::info!("POST /blogs failed to deserialize request: {}", e);
            conn.with_status(Status::BadRequest)
        }
    }
}

pub async fn post_blogs_post(conn: Conn) -> Conn {
    if conn.state::<User>().is_some() {
        if let Some(blog) = conn.state::<BlogResponse>().cloned() {
            conn.with_status(Status::Created).with_json(&blog)
        } else {
            conn
        }
    } else {
        conn
    }
}

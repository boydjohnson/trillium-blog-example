use crate::db::{self};
use sqlx::{Pool, Postgres};
use trillium::{Conn, Status};
use trillium_api::ApiConnExt;
use trillium_router::RouterConnExt;
use types::{
    blogs::{BlogResponse, NewBlogRequest},
    users::User,
};

pub async fn post_blogs_pre(mut conn: Conn) -> Conn {
    match conn.deserialize::<NewBlogRequest>().await {
        Ok(new_blog) => {
            if let Some(pool) = conn.state::<Pool<Postgres>>() {
                if let Some(user) = conn.state::<User>() {
                    match db::blogs::create_blog(pool, &new_blog, user).await {
                        Ok(resp) => conn.with_state(resp),
                        Err(e) => {
                            log::info!("POST /blogs error from database: {:?}", e);
                            conn.with_status(Status::InternalServerError)
                        }
                    }
                } else {
                    conn
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

pub async fn get_blogs(conn: Conn) -> Conn {
    if let Some(pool) = conn.state::<Pool<Postgres>>() {
        match db::blogs::list_blogs(pool).await {
            Ok(blogs) => conn.with_status(Status::Ok).with_json(&blogs),
            Err(e) => {
                log::warn!("GET /blogs database error: {:?}", e);
                conn.with_status(Status::InternalServerError)
            }
        }
    } else {
        conn.with_status(Status::InternalServerError)
    }
}

pub async fn get_blog_authors(conn: Conn) -> Conn {
    if let Some(blog_id) = conn.param("blog_id") {
        if let Ok(blog_id) = blog_id.parse::<i32>() {
            if let Some(pool) = conn.state::<Pool<Postgres>>() {
                match db::blogs::list_blog_authors(pool, blog_id).await {
                    Ok(users) => conn.with_status(Status::Ok).with_json(&users),
                    Err(_) => conn.with_status(Status::InternalServerError),
                }
            } else {
                conn.with_status(Status::InternalServerError)
            }
        } else {
            conn.with_status(Status::BadRequest)
        }
    } else {
        conn.with_status(Status::BadRequest)
    }
}

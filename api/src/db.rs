use crate::json_web_token::create_key;
use djangohashers::{make_password_with_algorithm, Algorithm};
use sqlx::{Pool, Postgres};
use std::borrow::Cow;
use types::users::{User, UsersRequest, UsersResponse};

pub async fn create_user(
    pool: &Pool<Postgres>,
    user: UsersRequest,
) -> Result<UsersResponse, DbError> {
    let password = make_password_with_algorithm(&user.password, Algorithm::BCryptSHA256);

    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (username, psswd) VALUES ( $1 , $2 ) RETURNING id, username;",
        user.username,
        password
    )
    .fetch_one(pool)
    .await?;

    let key = create_key(&user);

    let User { id, username } = user;

    Ok(UsersResponse { id, username, key })
}

#[derive(Debug)]
pub enum DbError {
    Conflict(String),
    ValidationError(String),
    Error(sqlx::Error),
}

const UNIQUE_CONSTRAINT: &str = "23505";
const FOREIGN_KEY_CONSTRAINT: &str = "23503";

impl From<sqlx::Error> for DbError {
    fn from(other: sqlx::Error) -> Self {
        match &other {
            sqlx::Error::Database(e) => {
                if e.code() == Some(Cow::from(UNIQUE_CONSTRAINT)) {
                    return DbError::Conflict("Unique constraint violated".to_string());
                } else if e.code() == Some(Cow::from(FOREIGN_KEY_CONSTRAINT)) {
                    return DbError::ValidationError("Missing Foreign Key object".to_string());
                }
                DbError::Error(other)
            }
            _ => DbError::Error(other),
        }
    }
}

use std::borrow::Cow;

pub mod blogs;
pub mod users;

#[derive(Debug)]
pub enum DbError {
    Conflict(String),
    ValidationError(String),
    NotFound,
    Error(sqlx::Error),
    PasswordNotCorrect,
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
            sqlx::Error::RowNotFound => DbError::NotFound,
            _ => DbError::Error(other),
        }
    }
}

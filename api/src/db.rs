use sqlx::{Pool, Postgres};
use types::users::User;

#[derive(Debug, Clone)]
pub struct State {
    pub pool: Pool<Postgres>,
}

#[derive(Debug, Clone)]
pub struct UserState {
    pub pool: Pool<Postgres>,
    pub user: User,
}

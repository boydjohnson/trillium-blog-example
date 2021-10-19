use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UsersRequest {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UsersResponse {
    id: i64,
    username: String,
    key: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    id: i64,
    username: String,
}

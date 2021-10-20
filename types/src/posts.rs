use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct NewPostRequest {
    title: String,
    text: String,
    tags: Vec<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostResponse {
    id: i32,
    title: String,
    slug: String,
    text: String,
    tags: Vec<String>,
}

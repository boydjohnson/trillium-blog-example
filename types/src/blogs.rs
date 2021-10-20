use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct NewBlogRequest {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BlogResponse {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub description: String,
}

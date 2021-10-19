use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct NewBlogRequest {
    title: String,
    description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BlogResponse {
    id: i64,
    slug: String,
    title: String,
    description: String,
}

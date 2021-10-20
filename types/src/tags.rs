use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TagResponse {
    id: i32,
    tag: String,
}

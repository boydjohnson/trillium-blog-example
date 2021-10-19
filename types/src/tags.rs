use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TagResponse {
    id: i64,
    tag: String,
}

use trillium_router::Router;

pub mod db;
pub mod json_web_token;
mod users;

pub fn router() -> Router {
    Router::new()
        .get("/", "Hello, World")
        .post("/login", "new_post")
        .get("/users", "List All the Users")
        .post("/users", users::post_users)
        .get("/users/self", "Return info about the current user.")
        .get("/blogs", "List all the blogs.")
        .post("/blogs", "Create a blog")
        .get(
            "/blogs/{blog_id}/authors",
            "List the authors of a blog post",
        )
        .post(
            "/blogs/{blog_id}/authors",
            "Create an invitation for a new author of a blog.",
        )
        .get("/blogs/{blog_id}/posts", "Get all the posts of a blog.")
        .post("/blogs/{blog_id}/posts", "Create a new post")
        .get("/blogs/{blog_id}/tags", "List all the tags for a blog.")
        .post("/blogs/{blog_id}/tags", "Create a new tag for a blog.")
}

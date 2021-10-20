use trillium_router::Router;

pub mod db;
pub mod json_web_token;
mod users;

pub fn router() -> Router {
    Router::new()
        .post("/login", users::login)
        .get("/users", (json_web_token::user_handler, users::get_users))
        .post("/users", users::post_users)
        .get(
            "/users/self",
            (json_web_token::user_handler, users::get_users_self),
        )
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

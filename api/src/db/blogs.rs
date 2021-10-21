use super::DbError;
use lazy_static::lazy_static;
use regex::Regex;
use sqlx::{Pool, Postgres};
use types::{
    blogs::{BlogResponse, NewBlogRequest},
    users::User,
};

lazy_static! {
    static ref REGEX: Regex = Regex::new(r#"[\s]+"#).unwrap();
}

fn sluggify(title: &str) -> String {
    REGEX.replace_all(&title.to_lowercase(), "-").to_string()
}

pub async fn create_blog(
    pool: &Pool<Postgres>,
    blog: &NewBlogRequest,
    user: &User,
) -> Result<BlogResponse, DbError> {
    let slug = sluggify(&blog.title);

    let mut transaction = pool.begin().await?;

    let slugs = sqlx::query!("SELECT slug FROM blogs WHERE slug LIKE $1 || '%';", slug)
        .fetch_all(&mut transaction)
        .await?;

    let slug = if let Some(max) = slugs
        .iter()
        .filter_map(|sl| sl.slug.split('-').last().map(ToString::to_string))
        .filter_map(|s| s.parse::<i32>().ok())
        .max()
    {
        format!("{}-{}", slug, max + 1)
    } else {
        format!("{}-{}", slug, 1)
    };

    match sqlx::query_as!(
        BlogResponse,
        "INSERT INTO blogs(title, descrip, slug) VALUES ($1, $2, $3) RETURNING id, title, slug, descrip as description;",
        blog.title,
        blog.description,
        slug
    )
    .fetch_one(&mut transaction)
    .await {
        Ok(resp) => {

            if let Err(e) = sqlx::query!("INSERT into blog_authors(blog_id, user_id) VALUES ($1, $2);", resp.id, user.id).execute(&mut transaction).await {
                transaction.rollback().await?;
                return Err(DbError::from(e));
            }


            transaction.commit().await?;
            Ok(resp)
        },
        Err(e) => {
            transaction.rollback().await?;
            Err(DbError::from(e))
        }
    }
}

pub async fn list_blogs(pool: &Pool<Postgres>) -> Result<Vec<BlogResponse>, DbError> {
    Ok(sqlx::query_as!(
        BlogResponse,
        "SELECT id, title, slug, descrip as description FROM blogs;"
    )
    .fetch_all(pool)
    .await?)
}

pub async fn list_blog_authors(pool: &Pool<Postgres>, blog_id: i32) -> Result<Vec<User>, DbError> {
    Ok(sqlx::query_as!(
        User,
        "SELECT u.id, u.username FROM users u JOIN blog_authors ba ON u.id = ba.user_id WHERE ba.blog_id = $1;",
        blog_id
    ).fetch_all(pool).await?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sluggify() {
        assert_eq!(
            &sluggify("The Very Hungry Caterpillar"),
            "the-very-hungry-caterpillar"
        );

        assert_eq!(&sluggify("The Roly Poly Puppy"), "the-roly-poly-puppy");
    }
}

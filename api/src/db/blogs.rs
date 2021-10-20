use super::DbError;
use lazy_static::lazy_static;
use regex::Regex;
use sqlx::{Pool, Postgres};
use types::blogs::{BlogResponse, NewBlogRequest};

lazy_static! {
    static ref REGEX: Regex = Regex::new(r#"[\s]+"#).unwrap();
}

fn sluggify(title: &str) -> String {
    REGEX.replace_all(&title.to_lowercase(), "-").to_string()
}

pub async fn create_blog(
    pool: &Pool<Postgres>,
    blog: &NewBlogRequest,
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
            transaction.commit().await?;
            Ok(resp)
        },
        Err(e) => {
            transaction.rollback().await?;
            Err(DbError::from(e))
        }
    }
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

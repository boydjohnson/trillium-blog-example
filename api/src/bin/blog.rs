use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use std::env::var;
use trillium::State;
use trillium_blog::router;
use trillium_logger::Logger;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let postgres_url = var("DATABASE_URL").unwrap();

    let pool = Pool::<Postgres>::connect(&postgres_url).await?;

    trillium_tokio::config()
        .with_port(8080)
        .with_host("127.0.0.1")
        .run_async((Logger::new(), State::new(pool), router()))
        .await;

    Ok(())
}

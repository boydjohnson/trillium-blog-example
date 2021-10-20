use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use std::env::var;
use trillium::State;
use trillium_blog::router;
use trillium_logger::{Logger, Target};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    env_logger::init();

    let postgres_url = var("DATABASE_URL").unwrap();

    let pool = Pool::<Postgres>::connect(&postgres_url).await?;

    trillium_tokio::config()
        .with_port(8080)
        .with_host("127.0.0.1")
        .run_async((
            Logger::new().with_target(Target::Logger(log::Level::Info)),
            State::new(pool),
            router(),
        ))
        .await;

    Ok(())
}

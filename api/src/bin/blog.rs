use trillium_blog::router;
use trillium_logger::Logger;

#[tokio::main]
async fn main() {
    trillium_tokio::config()
        .with_port(8080)
        .with_host("127.0.0.1")
        .run_async((Logger::new(), router()))
        .await;
}

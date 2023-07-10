use dotenvy::dotenv;
use shared_bookmarks::{run_server, setup_database};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();

    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL should be set in .env file");

    let db_pool = setup_database(database_url).await;

    run_server(listener, db_pool)?.await
}
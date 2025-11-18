mod infra;
mod adapters;
mod dto;
mod application;
mod domain;
mod utils;

use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::Builder::from_default_env()
        .format_timestamp_millis()
        .init();
    
    println!("Starting server...");

    infra::app::start_app().await
}
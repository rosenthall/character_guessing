use log::info;
use pretty_env_logger::env_logger;
use database;

#[tokio::main]
async fn main() {
    env_logger::init();

    info!("Program has started!");
}

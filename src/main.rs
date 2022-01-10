use actix_server::Server;
use anyhow::anyhow;
use log::info;

use crate::service::ActorServiceFactory;

mod echo;
mod read;
mod service;
mod write;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Err(_) = std::env::var("RUST_LOG") {
        std::env::set_var("RUST_LOG", "DEBUG");
    }
    env_logger::init();

    let address = "127.0.0.1:8000";
    info!("Server bind at: {}", address);
    Server::build()
        .bind("actix-server", address, || ActorServiceFactory)
        .map_err(|e| anyhow!(e))?
        .run()
        .await
        .map_err(|e| anyhow!(e))
}

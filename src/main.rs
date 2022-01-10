use std::net::TcpListener;

use actix_server::Server;
use anyhow::anyhow;
use log::info;
use tokio::net::TcpStream;

use crate::service::{ActorService, ActorServiceFactory};

mod echo;
mod read;
mod service;
mod write;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    match std::env::var("RUST_LOG") {
        Err(_) => std::env::set_var("RUST_LOG", "DEBUG"),
        _ => {}
    }
    env_logger::init();

    let address = "127.0.0.1:8000";
    let listener = TcpListener::bind(address).map_err(|e| anyhow!(e))?;
    info!("Server bind at: {}", address);

    Server::build()
        .listen("actix-server", listener, || ActorServiceFactory)
        .map_err(|e| anyhow!(e))?
        .run()
        .await
        .map_err(|e| anyhow!(e))
}

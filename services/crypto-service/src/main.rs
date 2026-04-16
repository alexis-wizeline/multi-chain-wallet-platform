use std::{env, net::SocketAddr};

use axum::serve;
use tokio::net::TcpListener;

mod app;
mod error;
mod models;
mod routes;
mod services;
mod state;

#[tokio::main]
async fn main() {
    let rpc_url = env::var("SOLANA_RPC_URL").expect("env variable SOLANA_RPC_URL to be set");

    let app = app::create_app(rpc_url).await;

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Signer service running on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();
}

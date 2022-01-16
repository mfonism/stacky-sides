use std::net::SocketAddr;

use axum::routing::get;
use axum::{Router, Server};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));

    let address = SocketAddr::from(([127, 0, 0, 1], 8000));
    Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> &'static str {
    "Hey there!"
}

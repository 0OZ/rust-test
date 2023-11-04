#[path = "router/routes"] mod routes;

use std::net::SocketAddr;
use axum::{Router, routing::get};


pub async fn serve(){
    let app = routes::create_router();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3404));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
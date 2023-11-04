use axum::{Json, Router, routing::get};
use axum::body::Body;
use axum::routing::post;

fn hello_world() -> Json<&'static str> {
    return Json("Hello, world!")
}

pub fn create_routes() -> Router<Body> {
  Router::new()
      .route("/", get(hello_world))
      // .route("/", post(handler()))
}

pub fn create_router() -> Router<Body> {
    Router::new().route("/", get(handler));
}
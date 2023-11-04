use std::net::SocketAddr;
use axum::ServiceExt;

mod routes;


pub async fn serve() {
    let app = routes::create_routes();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3404));

    super::utils::util::function();

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

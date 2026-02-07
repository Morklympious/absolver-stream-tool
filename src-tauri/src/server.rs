use axum::{
    Router,
    routing::get_service,
};

use tower_http::services::{ServeDir, ServeFile}
use std::net::SocketAddr;

pub async fn serve() {
    let build = "./build";
    let serve = ServeDir::new(build)
        .not_found_service(ServeFile::new(format!("{}/index./html", build)));

    let router = Router::new()
        .nest_service("/", serve);

    let address = SocketAddr::from(([127, 0, 0, 1], 5173));

    let binder = tokio::net::TcpListener::bind(address)
        .await
        .expect("Failed to bind HTTP server");

    axum::serve(binder, router)
    .await
    .expect("HTTP Server Failed");
}

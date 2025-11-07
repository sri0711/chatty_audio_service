use axum::{routing::get, Router};

use crate::routes::audio;

pub async fn run() {
    let route = Router::new()
        .route("/", get(|| async { "server is running" }))
        .nest_service("/audio", audio::audio_routes::bind_routes().await);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("server is started on http://localhost:3000/");
    axum::serve(listener, route.into_make_service()).await.unwrap()
}

use axum::{routing::get, Router};

pub async fn run() {
    let route = Router::new().route("/", get(|| async { "server is running" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("server is started on http://localhost:3000/");
    axum::serve(listener, route).await.unwrap()
}

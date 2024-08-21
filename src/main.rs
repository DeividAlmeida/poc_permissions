use axum;
mod controllers;
mod serializers;
mod routes;
mod db;

#[tokio::main]
async fn main() {
    let routes = routes::routes();
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, routes).await.unwrap();
}
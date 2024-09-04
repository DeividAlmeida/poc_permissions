use axum;
use dotenv::var;
mod controllers;
mod serializers;
mod routes;
mod db;
mod middleware;

#[tokio::main]
async fn main() {
    let port = var("PORT").expect("DB_URL must be set");
    let url = format!("0.0.0.0:{}", port);
    let routes = routes::routes();
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(url).await.unwrap();
    axum::serve(listener, routes).await.unwrap();
}
use axum::{routing::get, Router};
use crate::controllers::hello;

pub fn routes() -> Router {
  Router::new().route("/", get(hello))
}
use axum::{
  middleware::{self},
  Router,
  routing::get
};
use crate::controllers::hello;
use crate::middleware::Validations;

pub fn routes() -> Router {
  Router::new()
  .route("/", get(hello))
  .route("/oi", get(hello))
  .layer(middleware::from_fn(Validations::new))
}
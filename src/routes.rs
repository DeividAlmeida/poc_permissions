use axum::{
  middleware::{self},
  Router,
  routing::get
};
use crate::controllers::*;
use crate::middleware::Validations;

pub fn routes() -> Router {
  Router::new()
  .route("/", get(hello))
  .route("/menu", get(menu))
  .layer(middleware::from_fn(Validations::new))
}
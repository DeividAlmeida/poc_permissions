use axum::{
  middleware::{self}, routing::get, Router
};
use crate::controllers::*;
use crate::middleware::Validations;

pub fn routes() -> Router {
  Router::new()
  .route("/", 
    get(list_settings)
    .post(create_settings)
  )
  .route("/menu", get(menus))
  .route("/module", get(modules))
  .layer(middleware::from_fn(Validations::new))
  .route("/timeline", 
    get(list_timelines)
    .post(create_timelines)
  )
}
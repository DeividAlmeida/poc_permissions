use axum::{
   body::{to_bytes, Body}, extract::Request, response::Response, Json
};
use serde_json::Value;

use crate::serializers::*;

pub async fn list_settings(request:Request) -> Response {
  get_settings().await
}

pub async fn create_settings(Json(data): Json<Value>) -> Response {
  set_settings(data).await
}

pub async fn menus(request:Request) -> Response  {
  get_menus().await
}

pub async fn list_timelines(Json(data): Json<Value>) -> Response  {
  get_timelines(data).await
}

pub async fn create_timelines(Json(data): Json<Value>) -> Response  {
  set_timelines(data).await
}

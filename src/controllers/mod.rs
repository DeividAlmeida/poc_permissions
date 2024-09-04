use axum::{
  extract::Request, http::{header::{HeaderMap, HeaderValue}, request}, response::Json
};
use serde_json::{Value, json};
use crate::serializers::*;
struct ExtractUserAgent(HeaderValue);

pub async fn hello(request:Request) -> Json<Value> {
  get_key().await
}

pub async fn menu(request:Request) -> Json<Value> {
  get_menu().await
}
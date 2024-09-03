use axum::{
  extract::Request, http::{header::{HeaderMap, HeaderValue}, request}, response::Json
};
use serde_json::{Value, json};
use crate::serializers::get_key;
struct ExtractUserAgent(HeaderValue);

pub async fn hello(request:Request) -> Json<Value> {
  // dbg!(request.headers().get("user-agent"));
  get_key().await
}
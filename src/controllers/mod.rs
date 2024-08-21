use axum::response::Json;
use serde_json::{Value, json};
use crate::serializers::get_key;

pub async fn hello() -> Json<Value> {
  let key = get_key().await;
  return  match key {
    Ok(key) => Json(json!(key.unwrap())),
    Err(e) => {
      println!("error {:?}", e);
      return Json(json!({ "error": "error" }))
    }
  };
}
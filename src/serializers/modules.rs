use mongodb::bson::{doc, to_document};
use axum::{http::StatusCode, response::Json};
use serde_json::{Value, json};
use super::Queries;

pub(crate) trait Modules {
  async fn get (data: Value) -> (StatusCode, Json<Value>);
}

impl Modules for Queries {
  async fn get (data: Value) -> (StatusCode, Json<Value>) {
    let filter = to_document(&data).unwrap_or_else(|_| doc! {});
    let redis_key = format!("modules:{}", data.to_string());
    if let Ok(res) = Self::redis_get(&redis_key).await {
      let json_value = serde_json::from_str(&res).unwrap_or_else(|e| json!({"error": e.to_string()}));
      return (StatusCode::OK, Json(json_value));
    }

    match Self::mongodb_find(filter, "modules").await {
      Ok(res) => {
        let response = (StatusCode::OK, Json::<Value>(json!(res)));
        tokio::spawn(async move {
          Self::redis_set(&redis_key, json!(&res).to_string()).await 
        });
        response
      },
      Err(e) => {
        (StatusCode::INTERNAL_SERVER_ERROR, Json::<Value>(json!({"error": e.to_string()})))
      }
    }
  }
}
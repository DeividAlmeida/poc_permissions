use mongodb::bson::{doc, to_bson};
use axum::{http::StatusCode, response::Json};
use serde_json::{Value, json};
use super::Queries;

pub(crate) trait Settings {
  async fn get () -> (StatusCode, Json<Value>);
  async fn set (data: Value) -> (StatusCode, Json<Value>);
}

impl Settings for Queries {
  async fn get () -> (StatusCode, Json<Value>) {
    if let Ok(res) = Self::redis_get("settings").await {
      let json_value = serde_json::from_str(&res).unwrap_or_else(|e| json!({"error": e.to_string()}));
      return (StatusCode::OK, Json(json_value));
    }
    match Self::mongodb_find_one(doc! {"rule": "admin"}, "settings").await {
      Ok(res) => {
        let response = (StatusCode::OK, Json::<Value>(json!(res)));
        tokio::spawn(async move {
          Self::redis_set("settings", json!(&res).to_string()).await 
        });
        response
      },
      Err(e) => {
        (StatusCode::INTERNAL_SERVER_ERROR, Json::<Value>(json!({"error": e.to_string()})))
      }
    }
  }

  async fn set (data: Value) -> (StatusCode, Json<Value>) {
    match Self::mongodb_insert_one(doc! {"data": to_bson(&data).unwrap()}, "settings").await {
      Ok(res) => {
        (StatusCode::OK, Json::<Value>(json!(res)))
      },
      Err(e) => {
        (StatusCode::INTERNAL_SERVER_ERROR, Json::<Value>(json!({"error": e.to_string()})))
      }
    }
  }

}
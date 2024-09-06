use mongodb::bson::doc;
use axum::{http::StatusCode, response::Json};
use serde_json::{Value, json};
use super::Queries;

pub(crate) trait Menus {
  async fn get () -> (StatusCode, Json<Value>);
}

impl Menus for Queries {
  async fn get () -> (StatusCode, Json<Value>) {
    if let Ok(res) = Self::redis_get("menus").await {
      let json_value = serde_json::from_str(&res).unwrap_or_else(|e| json!({"error": e.to_string()}));
      return (StatusCode::OK, Json(json_value));
    }

    match Self::mongodb_find_one(doc! {"rule": "admin"}, "menus").await {
      Ok(res) => {
        let response = (StatusCode::OK, Json::<Value>(json!(res)));
        tokio::spawn(async move {
          Self::redis_set("menus", json!(&res).to_string()).await 
        });
        response
      },
      Err(e) => {
        (StatusCode::INTERNAL_SERVER_ERROR, Json::<Value>(json!({"error": e.to_string()})))
      }
    }
  }
}
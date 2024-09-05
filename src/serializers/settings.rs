use crate::db::mongo_connection;
use mongodb::bson::{ doc, Document };
use axum::{http::StatusCode, response::Json};
use serde_json::{Value, json};
use super::Cache;

pub(crate) trait Settings {
  async fn get () -> (StatusCode, Json<Value>);
  async fn get_settins_query () -> Result<Document, mongodb::error::Error>;
}

impl<'a> Settings for Cache<'a> {
  async fn get () -> (StatusCode, Json<Value>) {
    let mut cache = Cache::new("settings");
    match cache.get().await {
      Ok(res) => (StatusCode::OK, Json::<Value>(serde_json::from_str(&res).unwrap_or_else(|_| json!({"error": "Invalid JSON"})))),
      Err(_) => {
        match Self::get_settins_query().await {
          Ok(res) => {
            let data = (StatusCode::OK, Json::<Value>(json!(res)));
            tokio::spawn(async move {
              cache.set(res.to_string());
              cache.create().await 
            });
            data
          },
          Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json::<Value>(json!({"error": e.to_string()})))
          }
        }
      }
    }
  }

  async fn get_settins_query() -> Result<Document, mongodb::error::Error> {
    let config: crate::db::DatabaseConneceting = mongo_connection().await?;
    let collection = config.database.collection::<Document>("settings");

    tokio::spawn(async move {
        let _ = config.client.shutdown();
    });

    match collection.find_one(doc! {"rule": "admin"}).await {
        Ok(Some(doc)) => Ok(doc),
        Ok(None) => Ok(doc! {}),
        Err(e) => Err(e),
    }
}
}
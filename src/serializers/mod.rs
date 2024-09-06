mod settings;
mod menus;
mod tests;

use settings::Settings;
use menus::Menus;
use axum::response::{IntoResponse, Response};
use crate::db::{redis_connection, mongo_connection};
use mongodb::bson::{ doc, Document };
use redis::{Commands, RedisError};

struct Queries;

impl Queries {
  async fn redis_get(key: &str) -> Result<String, RedisError> {
    let mut conn = redis_connection().await?;
    conn.get(key)
  }

  async fn redis_set (key: &str, value: String) -> Result<String, RedisError> {
    let mut conn = redis_connection().await?;
    conn.set(key, value)
  }

  async fn mongodb_find_one(filter: Document, collection: &str) -> Result<Document, mongodb::error::Error> {
    let config: crate::db::DatabaseConneceting = mongo_connection().await?;
    let collection = config.database.collection::<Document>(collection);

    tokio::spawn(async move {
      let _ = config.client.shutdown();
    });

    match collection.find_one(filter).await {
      Ok(Some(doc)) => Ok(doc),
      Ok(None) => Ok(doc! {}),
      Err(e) => Err(e),
    }
  }
}

pub async fn get_settings() -> Response {
  <Queries as Settings>::get().await.into_response()
}

pub async fn get_menus() -> Response {
  <Queries as Menus>::get().await.into_response()
}


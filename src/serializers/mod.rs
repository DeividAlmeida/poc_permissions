mod settings;
mod menus;
mod timelines;
mod tests;

use serde_json::Value;
use settings::Settings;
use timelines::Timelines;
use menus::Menus;
use axum::response::{IntoResponse, Response};
use crate::db::{mongo_connection, redis_connection};
use mongodb::bson::{ doc,  DateTime, Document };
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
  
  async fn mongodb_aggregate(pipeline: Vec<Document>, collection: &str) -> Result<Vec<Document>, mongodb::error::Error> {
    let config: crate::db::DatabaseConneceting = mongo_connection().await?;
    let collection = config.database.collection::<Document>(collection);

    match collection.aggregate(pipeline).await {
      Ok(mut cursor) => {
        let mut res = Vec::new();
        while cursor.advance().await? {
          res.push(cursor.deserialize_current().unwrap_or_default());
        }
        Ok(res)
      },
      Err(e) => Err(e),
    }
  }
  
  async fn mongodb_insert_one(data: Document, collection: &str) -> Result<Document, mongodb::error::Error> {
    let config: crate::db::DatabaseConneceting = mongo_connection().await?;
    let collection = config.database.collection::<Document>(collection);

    tokio::spawn(async move {
      let _ = config.client.shutdown();
    });
    let mut data = data;
    data.insert("created_at", DateTime::now());
    data.insert("updated_at", DateTime::now());

    match collection.insert_one(data).await {
      Ok(_) => Ok(doc! {"success": true}),
      Err(e) => Err(e),
    }
  }

  async fn mongodb_insert_many(data: Vec<Document>, collection: &str) -> Result<Document, mongodb::error::Error> {
    let config: crate::db::DatabaseConneceting = mongo_connection().await?;
    let collection = config.database.collection::<Document>(collection);

    tokio::spawn(async move {
      let _ = config.client.shutdown();
    });

    match collection.insert_many(data).await {
      Ok(doc) => Ok(doc! {"success": true}),
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

pub async fn set_settings(data: Value) -> Response {
  <Queries as Settings>::set(data).await.into_response()
}

pub async fn get_timelines(data: Value) -> Response {
  <Queries as Timelines>::get(data).await.into_response()
}

pub async fn set_timelines(data: Value) -> Response {
  <Queries as Timelines>::set(data).await.into_response()
}


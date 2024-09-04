use crate::db::{ mongo_connection, redis_connection };
use axum::response::Json;
use serde_json::{Value, json};
use mongodb::bson::{ doc, Document };
use redis::{Commands, RedisError};

pub async fn get_key() -> Json<Value> {
  match cached_response().await {
    Ok(res) => {
      Json(serde_json::from_str(&res).unwrap())
    }
    _ => {
      let res = response().await;
      tokio::spawn(set_cache(res.to_string()));
      res
    }
  }
}

pub async fn get_menu() -> Json<Value> {
  menu().await
}

async fn cached_response()-> Result<String, RedisError> {
  match redis_connection().await {
    Ok(mut conn) => {
        let get: Result<String, RedisError> = conn.get("ALL_KEYS");
        let res: Result<String, RedisError> = match get  {
          Ok(val) => {
            Ok(val)
          },
          Err(e) => {
            Err(e)
          }
        };
        drop(conn);
        return res;
    },
    Err(e) => {
      Err(e)
    }
  }
}

async fn response () -> Json<Value> {
  match mongo_connection().await {
    Ok(config) => {
      let collection = config.database.collection::<Document>("settings");
      let result = collection.find_one(doc! {"rule":"admin"}).await.unwrap();
      tokio::spawn(async move {
        let _ = config.client.shutdown();
      });
      Json(json!(result))
    },
    Err(e) => {
      Json(json!({ "error": e.to_string() }))
    }
  }
}

async fn menu () -> Json<Value> {
  match mongo_connection().await {
    Ok(config) => {
      let collection = config.database.collection::<Document>("menus");
      let result = collection.find_one(doc! {"rule":"admin"}).await.unwrap();
      tokio::spawn(async move {
        let _ = config.client.shutdown();
      });
      Json(json!(result))
    },
    Err(e) => {
      Json(json!({ "error": e.to_string() }))
    }
  }
}

async fn set_cache (res: String) {
  match redis_connection().await {
    Ok(mut conn) => {
      let _: Result<String, RedisError> = conn.set_ex("ALL_KEYS", res, 30);
      drop(conn);
    },
    _ => {}
  }
}

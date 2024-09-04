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
    Err(e) => {
      let res = response().await;
      let _ = set_cache(res.to_string()).await;
      res
    }
  }
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
      dbg!(&e);
      Err(e)
    }
  }
}

async fn response () -> Json<Value> {
  match mongo_connection().await {
    Ok(config) => {
      let collection = config.database.collection::<Document>("settings");
      let result = collection.find_one(doc! {"rule":"admin"}).await.unwrap();
      
      config.client.shutdown().await;
      Json(json!(result))
    },
    Err(e) => {
      Json(json!({ "error": e.to_string() }))
    }
  }
}

async fn set_cache (res: String) {
  dbg!(&res);
  match redis_connection().await {
    Ok(mut conn) => {
      let test: Result<String, RedisError> = conn.set_ex("ALL_KEYS", res, 3600);
      dbg!(test);
      drop(conn);
    },
    Err(e) => {
      dbg!(e);
    }
  }
}

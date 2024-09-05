use axum::response::{IntoResponse, Response};
use redis::{Commands, RedisError};
use settings::Settings;
use crate::db::redis_connection;
mod settings;
mod menus;

struct Cache<'a> {
  key: &'a str,
  value: Option<String>
}

impl<'a> Cache<'a> {
  fn new(key: &'a str) -> Self {
    Cache {
      key,
      value: None
    }
  }

  fn set(&mut self, value: String) {
    self.value = Some(value);
  }

  async fn get(&self) -> Result<String, RedisError> {
    match redis_connection().await {
      Ok(mut conn) => {
        let res: Result<String, RedisError> = conn.get(&self.key);
        tokio::spawn(async move {
          drop(conn)
        });
        return res;
      },
      Err(e) => {
        Err(e)
      }
    }
  }

  async fn create (&self) {
    match redis_connection().await {
      Ok(mut conn) => {
        let _: Result<String, RedisError> = conn.set_ex(&self.key, self.value.as_ref().unwrap(), 30);
        tokio::spawn(async move {
          drop(conn)
        });
      },
      _ => {}
    }
  }
}

pub async fn get_settings<'a>() -> Response {
  <Cache<'a> as Settings>::get().await.into_response()
}

// pub async fn get_menu() -> (StatusCode, Json<Value>) {
//   match menus::get().await {
//     Ok(res) => {
//       (StatusCode::OK, Json(json!(res))) 
//     },
//     Err(e) => {
//       (StatusCode::BAD_REQUEST, Json(json!({ "error": e.to_string() })))
//     }
//   }
// }

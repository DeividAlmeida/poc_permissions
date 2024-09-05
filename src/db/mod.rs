use mongodb::{ Client, Database, error::Error as MongoError };
use std::io::{ Error, ErrorKind };
use redis::{Connection, RedisError};
use dotenv::var;
pub struct DatabaseConneceting {
  pub client: Client,
  pub database: Database,
}

pub async fn mongo_connection() -> Result<DatabaseConneceting, MongoError> {
  let url = var("DB_URL").expect("DB_URL must be set");
  let db = var("DB").expect("DB_NAME must be set");

match  Client::with_uri_str(&url).await {
    Ok(client, ) => {
      let database = client.database(&db);
      Ok(DatabaseConneceting { client,  database })
    }
    Err(e) => {
      Err(MongoError::from(Error::new(ErrorKind::NotConnected, e.to_string())))
    }
  }
}

pub async fn redis_connection() -> Result<Connection, RedisError> {
  let url = var("REDIS_URL").expect("DB_URL must be set");
  let client = redis::Client::open(url)?;
  let con = client.get_connection();
  con
}
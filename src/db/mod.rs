use mongodb::{ Client, Database };
use redis::{Connection, RedisError};
use dotenv::var;
pub struct DatabaseConneceting {
  pub client: Client,
  pub database: Database,
}

pub async fn mongo_connection() -> Result<DatabaseConneceting, mongodb::error::Error> {
  let url = var("DB_URL").expect("DB_URL must be set");
  let db = var("DB").expect("DB_NAME must be set");
  let client = Client::with_uri_str(&url).await.unwrap();
  let database = client.database(&db);
  Ok(DatabaseConneceting { client, database })
}

pub async fn redis_connection() -> Result<Connection, RedisError> {
  let url = var("REDIS_URL").expect("DB_URL must be set");
  let client = redis::Client::open(url)?;
  let con = client.get_connection();
  con
}
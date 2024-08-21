use mongodb::{ Client, Database };
pub struct DatabaseConneceting {
  pub client: Client,
  pub database: Database,
}

pub async fn connect() -> Result<DatabaseConneceting, mongodb::error::Error> {
  let uri = dotenv::var("DB_URL").expect("DB_URL must be set");
  let db = dotenv::var("DB").expect("DB_NAME must be set");
  let client = Client::with_uri_str(&uri).await.unwrap();
  let database = client.database(&db);
  Ok(DatabaseConneceting { client, database })
}
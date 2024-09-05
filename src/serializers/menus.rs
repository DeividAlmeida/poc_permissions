use crate::db::mongo_connection;
use mongodb::bson::{ doc, Document };

pub async fn get () -> Result<Document, mongodb::error::Error> {
  match mongo_connection().await {
    Ok(config) => {
      let collection = config.database.collection::<Document>("menus");
      let result = collection.find_one(doc! {"rule":"user"}).await.unwrap();
      tokio::spawn(async move {
        let _ = config.client.shutdown();
      });
      match result {
        Some(document) => {
          Ok(document)
        },
        None => {
          Ok(doc! {})
        }
      }
    },
    Err(e) => {
      Err(e)
    }
  }
}
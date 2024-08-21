use crate::db::connect;
use axum::response;
use mongodb::{bson::{ doc, oid::ObjectId, DateTime, Document }, error::Error, Database, Cursor, results::UpdateResult };

pub async fn get_key() -> Result<Option<Document>, Error> {
  let document = match connect().await {
    Ok(config) => {
      print!("db connected");
       let res = config.database.collection::<Document>("settings_keys").find_one(doc! { 
        "constantName": "CAPEX_ANUAL_MANDATORY" 
      }).await.unwrap();
      config.client.shutdown().await;
      Ok(res)
    },
    Err(e) => {
      println!("db connection error {:?}", e);
      Err(e)
    }  
  };
  document
}
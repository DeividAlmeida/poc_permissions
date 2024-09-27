use std::f32::consts::E;

use mongodb::bson::{doc, to_bson, DateTime, Document};
use axum::{body::Body, http::{Error, StatusCode}, response::Json};
use serde_json::{Value, json};
use super::Queries;

pub(crate) trait Timelines {
  async fn get (data: Value) -> (StatusCode, Json<Value>);
  async fn set (data: Value) -> (StatusCode, Json<Value>);
  async fn timeline_data(data: &Value) -> Result<Vec<Document>, String>;
}

impl Timelines for Queries {
  async fn get (data: Value) -> (StatusCode, Json<Value>) {
    let pipeline = vec![
      doc! {
        "$lookup": {
          "from": "maintenance_orders",
          "localField": "mo",
          "foreignField": "_id",
          "as": "mo"
        }
      },
      doc! {
        "$unwind": {
          "path": "$mo",
          "preserveNullAndEmptyArrays": false
        }
      },
      doc!{ 
        "$sort": { 
          "timestamp": 1
        } 
      },
      doc! {
        "$match": to_bson(&data).unwrap()
      },
      doc! { 
        "$project": { 
          "mo": 0,
          "timestamp": 0,
          "created_at": 0,
          "updated_at": 0
        } 
      }
    ];
    match Self::mongodb_aggregate(pipeline, "timelines").await {
      Ok(res) => {
        let response = (StatusCode::OK, Json::<Value>(json!(res)));
        tokio::spawn(async move {
          Self::redis_set("timelines", json!(&res).to_string()).await 
        });
        response
      },
      Err(e) => {
        (StatusCode::INTERNAL_SERVER_ERROR, Json::<Value>(json!({"error": e.to_string()})))
      }
    }
  }

  async fn set (data: Value) -> (StatusCode, Json<Value>) {
    let timeline_data = match Self::timeline_data(&data).await {
      Ok(res) => res,
      Err(e) => {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json::<Value>(json!({"error": e})))
      } 
    };
    
    match Self::mongodb_insert_many(timeline_data, "timelines").await {
      Ok(res) => {
        (StatusCode::OK, Json::<Value>(json!(res)))
      },
      Err(e) => {
        (StatusCode::INTERNAL_SERVER_ERROR, Json::<Value>(json!({"error": e.to_string()})))
      }
    }
  }

  async fn timeline_data(data: &Value) -> Result<Vec<Document>, String> {
    let data_ref = data.get("ref")
      .ok_or("Missing 'ref' field")?
      .as_str()
      .ok_or("'ref' is not a string")?;

    let branch_name = data_ref.split('/').last()
      .ok_or("Failed to extract branch_name from 'ref'")?;

    let mo = Self::mongodb_find_one(doc!{"branch_name": branch_name}, "maintenance_orders").await
      .map_err(|e| e.to_string()).unwrap_or_default();
    
    let mo_id = mo.get("_id")
      .ok_or("Missing '_id' field")?
      .as_object_id()
      .ok_or("'_id' is not an ObjectId")?;
    
    let commits = data.get("commits")
      .ok_or("Missing 'commits' field")?
      .as_array()
      .ok_or("'commits' is not an array")?;

    if commits.is_empty() {
      return Err("No commits found".to_string());
    }

    let commit_docs: Vec<Document> = commits.iter().filter_map(|commit| {
      let author = commit.get("author")
        .and_then(|a| a.get("name"))
        .and_then(|n| n.as_str())?;

        let message = commit.get("message")
          .and_then(|m| m.as_str())?;

        let timestamp = commit.get("timestamp")
          .and_then(|t| t.as_str())?;

        let url = commit.get("url")
          .and_then(|u| u.as_str())?;

        Some(doc! {
          "mo": mo_id,
          "author": author.to_string(),
          "message": message.to_string(),
          "timestamp": DateTime::parse_rfc3339_str(timestamp).unwrap(),
          "date": timestamp,
          "url": url.to_string(),
          "created_at": DateTime::now(),
          "updated_at": DateTime::now()
        })
    }).collect();

    Ok(commit_docs)

  }

}
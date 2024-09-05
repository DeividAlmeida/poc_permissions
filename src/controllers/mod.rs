use axum::{
  extract::Request,
  response::Response
};
use crate::serializers::*;

pub async fn settings(request:Request) -> Response {
  get_settings().await
}

// pub async fn menu(request:Request) -> (StatusCode, Json<Value>)  {
//   get_menu().await
// }
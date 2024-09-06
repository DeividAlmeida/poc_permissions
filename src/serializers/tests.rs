mod test {
  use tokio;
  use mongodb::bson::doc;
  use crate::serializers::Queries;

  #[ignore]
  #[tokio::test]
  async fn test_redis_set_ok() {
      let result = Queries::redis_set("test_key", "test_value".to_string()).await;
      assert!(result.is_ok());
  }

  #[ignore]
  #[tokio::test]
  async fn test_redis_get_ok() {
      let result = Queries::redis_get("test_key").await;
      assert!(result.is_ok());
      assert_eq!(result.unwrap(), "test_value".to_string());
  }

  #[ignore]
  #[tokio::test]
  async fn test_redis_get_error() {
      let result = Queries::redis_get("nonexistent_key").await;
      assert!(result.is_err());
  }

  #[ignore]
  #[tokio::test]
  async fn test_mongodb_find_one_not_found() {
      let filter = doc! {"nonexistent": "value"};
      let result = Queries::mongodb_find_one(filter, "settings").await;
      assert!(result.is_ok());
      let doc = result.unwrap();
      assert!(doc.is_empty());
  }
  
  #[ignore]
  #[tokio::test]
  async fn test_mongodb_find_one_error() {
      let result = Queries::mongodb_find_one(doc! {"key": "value"}, "invalid_collection").await;
      assert!(result.is_ok());
      let doc = result.unwrap();
      assert!(doc.is_empty());
  }
}

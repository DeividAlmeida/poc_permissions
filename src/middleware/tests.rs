#[cfg(test)]
mod tests {
    use crate::middleware::Validations;
    use axum::{
        body::Body,
        http::{HeaderValue, Request, StatusCode},
    };

    #[ignore]
    #[tokio::test]
    async fn test_valid_origin_and_auth() {
      let request = Request::builder()
      .header("authorization", HeaderValue::from_static("1234"))
      .header("app-origin", HeaderValue::from_static("api"))
      .body(Body::empty())
      .unwrap();

      let response = Validations {
        auth: request.headers().get("authorization"),
        origin: request.headers().get("app-origin"),
      };

      let (status, body) = response.origin_validation();
      assert_eq!(status, StatusCode::OK);
      assert_eq!(body, "OK");
    }

    #[ignore]
    #[tokio::test]
    async fn test_invalid_origin() {
      let request = Request::builder()
        .header("authorization", HeaderValue::from_static("123"))
        .header("app-origin", HeaderValue::from_static("apis"))
        .body(Body::empty())
        .unwrap();

        let response = Validations {
          auth: request.headers().get("authorization"),
          origin: request.headers().get("app-origin"),
        };
  
        let (status, body) = response.origin_validation();
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(body, "Invalid origin");
    }

    #[ignore]
    #[tokio::test]
    async fn test_missing_origin() {
      let request = Request::builder()
        .header("authorization", HeaderValue::from_static("123"))
        .body(Body::empty())
        .unwrap();

        let response = Validations {
          origin: request.headers().get("app-origin"),
          auth: request.headers().get("authorization"),
        };

        let (status, body) = response.origin_validation();
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(body, "Missed origin");
    }

    #[ignore]
    #[tokio::test]
    async fn test_invalid_auth() {
      let request = Request::builder()
        .header("authorization", "Token token=\"123\"")
        .header("app-origin", HeaderValue::from_static("api"))
        .body(Body::empty())
        .unwrap();

      let response = Validations {
        origin: request.headers().get("app-origin"),
        auth: request.headers().get("authorization"),
      };

      let (status, body) = response.auth_validation();
      assert_eq!(status, StatusCode::UNAUTHORIZED);
      assert_eq!(body, "Invalid token");
    }

    #[ignore]
    #[tokio::test]
    async fn test_missing_auth() {
      let request = Request::builder()
        .header("app-origin", HeaderValue::from_static("api"))
        .body(Body::empty())
        .unwrap();

        
      let response = Validations {
        origin: request.headers().get("app-origin"),
        auth: request.headers().get("authorization"),
      };

      let (status, body) = response.auth_validation();
      assert_eq!(status, StatusCode::UNAUTHORIZED);
      assert_eq!(body, "Missed token");
    }
}

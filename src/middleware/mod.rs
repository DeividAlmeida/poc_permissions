use axum::{body::Body, extract::Request, http::{HeaderValue, StatusCode}, middleware::Next, response::IntoResponse};
pub(crate) struct Validations<'b> {
  origin: Option<&'b HeaderValue>,
  auth: Option<&'b HeaderValue>,
}

impl<'a> Validations <'_> {
  pub async fn new(  
    req: Request,
    next: Next
  ) -> Result<impl IntoResponse, Body> {

    let auth = req.headers().get("authorization");
    let origin = req.headers().get("app-origin");
    let validation = Validations { auth, origin };

    let (origin_status, origin_res)=  validation.origin_validation();
    let (auth_status, aut_res) = validation.auth_validation();
    
    match (origin_status, auth_status) {
      ( StatusCode::UNAUTHORIZED, _ )  => Ok((origin_status, origin_res).into_response()), 
      ( _, StatusCode::UNAUTHORIZED ) => Ok((auth_status, aut_res).into_response()),
      _ =>{
        let res = next.run(req).await;
        Ok(res)
      }
    }
  }

  fn origin_validation(&self) -> ( StatusCode, &'a str ){
    match self.origin {
      Some(origin) => { 
        if origin == "api" || origin == "mobile" {
          return (
            StatusCode::OK,
            "OK",
          )
        }
        return ( StatusCode::UNAUTHORIZED, "Invalid origin" );
      },
      None => (StatusCode::UNAUTHORIZED, "Missed origin"),
    }
  }

  fn auth_validation(&self) -> ( StatusCode, &'a str ) {
    match self.auth {
      Some(auth) => {
        if auth != "123" {
          return (StatusCode::OK, "OK")
        }
        return (StatusCode::UNAUTHORIZED, "Invalid token")
      },
      None => (StatusCode::UNAUTHORIZED, "Missed token"),
    }
  }
}
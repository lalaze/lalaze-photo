use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MyResponse<T> {
  pub result: String,
  pub message: String,
  pub data: Option<T>,
}

#[macro_export]
macro_rules! response {
  () => {
    HttpResponse::Ok().json(MyResponse {
      result: "0".to_string(),
      message: "error".to_string(),
      data: None
    })
  };
  ($message:expr) => {
    HttpResponse::Ok().json(MyResponse {
      result: "0".to_string(),
      message: $message.to_string(),
      data: None
    })
  };
  ($code:expr, $message:expr) => {
    HttpResponse::Ok().json( MyResponse {
      result: $code.to_string(),
      message: $message.to_string(),
      data: None
    })
  };
  ($code:expr, $message:expr, $data:expr) => {
    HttpResponse::Ok().json(MyResponse {
      result: $code.to_string(),
      message: $message.to_string(),
      data: $data
    })
  };
}
pub(crate) use response; 
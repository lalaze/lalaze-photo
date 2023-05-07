use actix_web::middleware::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::{
    dev,
    http::{header, StatusCode},
    web, App, HttpResponse, HttpServer, Result,
};
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
struct Response<T> {
    status: u16,
    message: String,
    data: Option<T>,
}

pub async fn response_middleware(
  mut res: dev::ServiceResponse<B>
) {
  let a = res.response_mut();
   
}
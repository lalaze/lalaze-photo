use crate::{repository::mongodb_repos::MongoRepo};
use actix_web::{
  post,
  web::{Data, Json, BytesMut, Payload},
  HttpResponse,
};
use futures_util::StreamExt as _;

#[post("/upload")]
pub async fn upload_file(db: Data<MongoRepo>, fileName: String, mut file: Payload) -> HttpResponse {
    let mut bytes = BytesMut::new();
    while let Some(item) = file.next().await {
        bytes.extend_from_slice(&item.unwrap());
    }
    let result = db.uploadFile(fileName, bytes).await;
    match result {
        Ok(_) => HttpResponse::Ok().json("okok"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
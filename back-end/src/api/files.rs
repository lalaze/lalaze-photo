use crate::{repository::mongodb_repos::MongoRepo};
use actix_multipart::Multipart;
use serde::Deserialize;
use actix_web::{
  post,
  get,
  web::{Data, Query},
  HttpResponse,
  Error,
  error
};
use crate::models::photo::Photo;
use futures_util::StreamExt as _;

#[post("/upload")]
pub async fn upload_file(db: Data<MongoRepo>, mut payload: Multipart) -> Result<HttpResponse, Error>  {
    let mut file_content: Option<Vec<u8>> = None;
    let mut file_name: Option<String> = None;

    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();

        let content_disposition = field.content_disposition();
        let name = content_disposition.get_name().unwrap().to_string();

        if name == "file" {
            // Read the content of the field into a buffer
            let mut buffer = Vec::new();
            while let Some(chunk) = field.next().await {
                buffer.extend_from_slice(&chunk.unwrap());
            }
            file_content = Some(buffer);
        } else if name == "file_name" {
            // Read the content of the field into a string
            let mut buffer = String::new();
            while let Some(chunk) = field.next().await {
                buffer.push_str(std::str::from_utf8(&chunk.unwrap()).unwrap());
            }
            file_name = Some(buffer);
        }
    }

    let file_content = file_content.ok_or_else(|| error::ErrorBadRequest("missing file field"))?;
    let file_name = file_name.ok_or_else(|| error::ErrorBadRequest("missing file_name field"))?;

    db.upload_file(file_name, file_content).await.expect("uplaod field");

    Ok(HttpResponse::Ok().body("upload done"))

}

#[get("upload_file_path")]
pub async fn upload_file_path(db: Data<MongoRepo>, info: Query<Photo>) -> Result<HttpResponse, Error>  {
  db.crate_photo(info.name.clone(), info.location.clone()).await.expect("create field");
  Ok(HttpResponse::Ok().body("upload done"))
}

#[derive(Deserialize)]
pub struct Info {
  file_path: String,
}

#[get("upload_file_dir")]
pub async fn upload_file_dir(db: Data<MongoRepo>, info: Query<Info>) -> Result<HttpResponse, Error>  {
  let path = info.file_path.clone();

  db.crate_photo_dir(&path).await.expect("create field");

  Ok(HttpResponse::Ok().body("upload done"))
}


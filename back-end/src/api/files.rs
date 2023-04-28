use crate::{repository::mongodb_repos::MongoRepo};
use actix_multipart::Multipart;
use actix_web::{
  post,
  web::{Data, Form },
  HttpResponse,
  Error,
  error
};
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

    db.uploadFile(file_name, file_content).await;

    print!("{}", "1234");

    Ok(HttpResponse::Ok().finish())

}
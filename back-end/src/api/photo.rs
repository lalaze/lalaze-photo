use crate::{repository::mongodb_repos::MongoRepo};
use crate::{api::response::MyResponse};
use crate::{api::auth::auth_error};
use actix_multipart::Multipart;
use serde::Deserialize;
use crate::api::user_data::UserData;
use actix_web::{
  post,
  get,
  web::{Data, Query, Path},
  HttpResponse,
  error
};
use crate::models::photo::Photo;
use futures_util::StreamExt as _;
use regex::Regex;

#[post("/upload")]
pub async fn upload_file(db: Data<MongoRepo>, user: Option<UserData>, mut payload: Multipart) -> HttpResponse  {
  if let Some(user) = user {
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

    let file_content = file_content.ok_or_else(|| error::ErrorBadRequest("missing file field")).unwrap();
    let file_name = file_name.ok_or_else(|| error::ErrorBadRequest("missing file_name field")).unwrap();

    db.upload_file(file_name, file_content).await.expect("uplaod field");

    let result: MyResponse<String> = MyResponse {
      result: "0".to_string(),
      message: "upload done".to_string(),
      data: None
    };

    HttpResponse::Ok().json(result)
  } else {
    auth_error()
  }
}

#[get("upload_file_path")]
pub async fn upload_file_path(db: Data<MongoRepo>, user: Option<UserData>, info: Query<Photo>) -> HttpResponse {
  if let Some(user) = user {
    db.crate_photo(info.name.clone(), info.location.clone()).await.expect("create field");
    HttpResponse::Ok().body("upload done")
  } else {
    auth_error()
  }
}

#[derive(Deserialize)]
pub struct Upload_Info {
  file_path: String,
}

#[get("upload_file_dir")]
pub async fn upload_file_dir(db: Data<MongoRepo>, user: Option<UserData>, info: Query<Upload_Info>) -> HttpResponse  {
  if let Some(user) = user {
    let path = info.file_path.clone();

    db.crate_photo_dir(&path).await.expect("create field");
  
  
    let result: MyResponse<String> = MyResponse {
      result: "0".to_string(),
      message: "upload done".to_string(),
      data: None
    };
  
    HttpResponse::Ok().json(result)
  } else {
    auth_error()
  }
}

#[derive(Deserialize)]
pub struct Get_Info {
  offset: u64,
  limit: i64
}


#[get("get_photos")]
pub async fn get_photos(db: Data<MongoRepo>, user: Option<UserData>, info: Query<Get_Info>) -> HttpResponse  {
  if let Some(user) = user {
    let photos = db.get_photos(info.offset, info.limit).await;
    match photos {
      Ok(photos) => {
        let result = MyResponse {
          result: "0".to_string(),
          message: "get done".to_string(),
          data: Some(photos)
        };
        HttpResponse::Ok().json(result)
      },
      Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
  } else {
    auth_error()
  }
}

#[derive(Debug, Deserialize)]
pub struct Update_Info {
  id: String,
  name: String,
  location: String,
  tag: String
}

#[get("update_photo")]
pub async fn update_photo(db: Data<MongoRepo>, user: Option<UserData>, info: Query<Update_Info>) -> HttpResponse  {
  if let Some(user) = user {
    let id: String = info.id.clone();
    let tags: Vec<Option<String>>;
    let re = Regex::new(r"^[^,]*(,[^,]*)*$").unwrap();
    if re.is_match(&info.tag) {
      tags = info.tag.split(',').map(|x| if x.is_empty() { None } else { Some(x.to_string()) })
      .collect();
    } else {
      return HttpResponse::InternalServerError().body("tag is illegality")
    };
    let data = Photo {
      id: Some(info.id.clone()),
      name: info.name.to_owned(),
      location: info.location.to_owned(),
      tag: tags
    };
    let update_result = db.edit_photos(&id, data).await;
    match update_result {
      Ok(update) => {
          if update.matched_count == 1 {
              let updated_photo_info = db.get_photo(&id).await;
              return match updated_photo_info {
                  Ok(photo) => {
                    let result = MyResponse {
                      result: "0".to_string(),
                      message: "update done".to_string(),
                      data: Some(photo)
                    };
                    HttpResponse::Ok().json(result)
                  },
                  Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
              };
          } else {
              return HttpResponse::NotFound().body("No photo found with specified ID");
          }
      }
      Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
  } else {
    auth_error()
  }
}

#[derive(Deserialize)]
pub struct Delete_Info {
  id: String
}


#[get("/delte_photo")]
pub async fn delte_photo(db: Data<MongoRepo>, user: Option<UserData>, info: Query<Delete_Info>) -> HttpResponse  {
  if let Some(user) = user {
    let id: String = info.id.clone();
  if id.is_empty() {
    return HttpResponse::BadRequest().body("invalid ID");
  };
  let result = db.delete_photo(&id).await;
  match result {
      Ok(res) => {
          if res.deleted_count == 1 {
              let result: MyResponse<String> = MyResponse {
                result: "0".to_string(),
                message: "oto successfully deleted!".to_string(),
                data: None
              };
              return HttpResponse::Ok().json(result);
          } else {
              return HttpResponse::NotFound().json("photo with specified ID not found!");
          }
      }
      Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
  }
  } else {
    auth_error()
  }
}

use crate::{repository::mongodb_repos::MongoRepo};
use crate::{api::response::MyResponse};
use actix_web::{
  post,
  get,
  web::{Data, Query, Path},
  HttpResponse,
  error
};
use serde::Deserialize;
use crate::models::tag::Tag;

#[derive(Deserialize)]
pub struct Tag_Info {
  name: String,
  color: String
}

#[get("/add_tag")]
pub async fn add_tag(db: Data<MongoRepo>, info: Query<Tag_Info>) -> HttpResponse  {
  let id: String = info.name.clone();
  if id.is_empty() {
    return HttpResponse::BadRequest().body("invalid ID");
  };
  let result = db.add_tag(&id, &info.color).await;
  match result {
    Ok(()) => {
      let result: MyResponse<String> = MyResponse {
        result: "0".to_string(),
        message: "add done".to_string(),
        data: None
      };
  
      HttpResponse::Ok().json(result)
    },
    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
  }
}

#[derive(Deserialize)]
pub struct Update_Tag_Info {
  id: String,
  name: String,
  color: String
}

#[get("/update_tag")]
pub async fn update_tag(db: Data<MongoRepo>, info: Query<Update_Tag_Info>) -> HttpResponse  {
  let id: String = info.id.clone();
  let update_result = db.edit_tag(&info.id, &info.name, &info.color).await;
  match update_result {
    Ok(update) => {
        if update.matched_count == 1 {
            let updated_tag_info = db.get_photo(&id).await;
            return match updated_tag_info {
              Ok(tag) => {
                let result = MyResponse {
                  result: "0".to_string(),
                  message: "update done".to_string(),
                  data: Some(tag)
                };
            
                return HttpResponse::Ok().json(result)
              },
              Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            };
        } else {
            return HttpResponse::NotFound().body("No tag found with specified ID");
        }
    }
    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
  }
}

#[derive(Deserialize)]
pub struct Delete_Info {
  id: String
}


#[get("/delete_tag")]
pub async fn delte_tag(db: Data<MongoRepo>, info: Query<Delete_Info>) -> HttpResponse  {
  let id: String = info.id.clone();
  if id.is_empty() {
    return HttpResponse::BadRequest().body("invalid ID");
  };
  let result = db.delete_tag(&id).await;
  match result {
      Ok(res) => {
          if res.deleted_count == 1 {
            let result: MyResponse<String> = MyResponse {
              result: "0".to_string(),
              message: "tag successfully deleted!".to_string(),
              data: None
            };
        
            return HttpResponse::Ok().json(result);
          } else {
              return HttpResponse::NotFound().json("tag with specified ID not found!");
          }
      }
      Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
  }
}
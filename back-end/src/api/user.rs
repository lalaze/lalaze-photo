use crate::{repository::mongodb_repos::MongoRepo};
use crate::{api::response::MyResponse};
use crate::{api::auth::auth_error};
use crate::api::user_data::UserData;
use actix_web::{
  post,
  get,
  web::{Data, Query, Path},
  HttpResponse,
};
use serde::Deserialize;


#[derive(Deserialize)]
pub struct User_Info {
  username: String,
  password: String
}

#[get("/add_user")]
pub async fn add_user(db: Data<MongoRepo>, user: Option<UserData>, info: Query<User_Info>) -> HttpResponse  {
  let name: String = info.username.clone();
  if name.is_empty() {
    return HttpResponse::BadRequest().body("invalid name");
  };
  let mut hasher = md5::Context::new();
  // 将密码传递给计算器
  hasher.consume(info.password.as_bytes());
  // 计算 MD5 哈希值
  let result = hasher.compute();
  // 将哈希值转换为字符串表示
  let hashed_password = format!("{:x}", result);
  let result = db.create_user(&name, &hashed_password).await;
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
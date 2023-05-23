use crate::{repository::mongodb_repos::MongoRepo};
use actix_web::{web};
use crate::{api::response::MyResponse};
use serde::{Deserialize, Serialize};
use crate::api::user_data::UserData;
use actix_web::{
  post,
  get,
  web::{Data, Query, Path},
  HttpResponse,
};


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

#[derive(Serialize, Deserialize)]
pub struct LoginDTO {
  pub username: String,
  pub password: String,
}

/// 登陆
#[post("/login")]
pub async fn login(db: Data<MongoRepo>, body: web::Json<LoginDTO>) -> HttpResponse {
  print!("123");
  // 检查密码对不对
  let mut hasher = md5::Context::new();
  // 将密码传递给计算器
  hasher.consume(body.password.as_bytes());
  // 计算 MD5 哈希值
  let result = hasher.compute();
  // 将哈希值转换为字符串表示
  let hashed_password = format!("{:x}", result);
  let user_data = db.get_user(&body.username.clone()).await.unwrap();

  if let Some(user) = user_data {
    if hashed_password == user.password {
      let token = crate::api::auth::create_jwt(&user.userName);
      let result: MyResponse<String> = MyResponse {
        result: "0".to_string(),
        message: "login success".to_string(),
        data: Some(token)
      };
      HttpResponse::Ok().json(result)
    } else {
      let result: MyResponse<String> = MyResponse {
        result: "0".to_string(),
        message: "error password".to_string(),
        data: None
      };
      HttpResponse::Ok().json(result)
    }
  } else {
    let result: MyResponse<String> = MyResponse {
      result: "0".to_string(),
      message: "user error".to_string(),
      data: None
    };
    HttpResponse::Ok().json(result)
  }
}
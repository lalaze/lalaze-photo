use actix_web::{web, get, App, HttpResponse, HttpServer, Responder, web::Data, post};
mod api; 
mod models;
mod repository;
use repository::mongodb_repos::MongoRepo;
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
// use actix_web_httpauth::{extractors::bearer::BearerAuth, middleware::HttpAuthentication};

// async fn ok_validator(
//   req: ServiceRequest,
//   credentials: BearerAuth,
// ) -> Result<ServiceRequest, (Error, ServiceRequest)> {
//   eprintln!("{credentials:?}");
//   Ok(req)
// }



#[get("/")]
async fn hello() -> impl Responder {
  HttpResponse::Ok().json("Hello from rust and mongoDB")
}

#[derive(Serialize, Deserialize)]
struct LoginDTO {
  pub username: String,
  pub password: String,
}

/// 登陆
#[post("/login")]
async fn login(db: Data<MongoRepo>, body: web::Json<LoginDTO>) -> impl Responder {
  // 检查密码对不对
  let mut hasher = md5::Context::new();
  // 将密码传递给计算器
  hasher.consume(body.password.as_bytes());
  // 计算 MD5 哈希值
  let result = hasher.compute();
  // 将哈希值转换为字符串表示
  let hashed_password = format!("{:x}", result);
  let user_data: Option<crate::api::user_data::UserData> = db.get_user(&body.username.clone());

  if let Some(user) = user_data {
    if hashed_password == user
  } else {
    HttpResponse::Ok().json("user error")
  }
  
  // let token = api::auth::create_jwt(&body.username);
  HttpResponse::Ok().json(token)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let db = MongoRepo::init().await;
  let db_data = Data::new(db);
  HttpServer::new(move || {
      App::new()
          .wrap(Cors::permissive())
          .app_data(db_data.clone())
          .service(api::photo::upload_file)
          .service(api::photo::upload_file_path)
          .service(api::photo::upload_file_dir)
          .service(api::photo::get_photos)
          .service(api::photo::update_photo)
          .service(api::photo::delte_photo)
          .service(api::tag::add_tag)
          .service(api::tag::update_tag)
          .service(api::tag::delte_tag)
          .service(login)
          .service(api::user::add_user)
  })
  .bind(("127.0.0.1", 8083))?
  .run()
  .await
}
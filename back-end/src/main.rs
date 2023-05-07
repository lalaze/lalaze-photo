use actix_web::{get, App, HttpResponse, HttpServer, Responder, web::Data};
mod api; 
mod models;
mod repository;
use repository::mongodb_repos::MongoRepo;
use api::photo;
use std::{fs, path::Path};
use std::path::PathBuf;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello from rust and mongoDB")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  // let mut stack = vec![PathBuf::from("./test")];
  // print!("{:?}", stack);

  let db = MongoRepo::init().await;
  let db_data = Data::new(db);
  HttpServer::new(move || {
      App::new()
          .app_data(db_data.clone())
          .service(api::photo::upload_file)
          .service(api::photo::upload_file_path)
          .service(api::photo::upload_file_dir)
          .service(api::photo::get_photos)
          .service(api::photo::update_photo)
          .service(api::photo::delte_photo)
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
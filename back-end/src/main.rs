use actix_web::{get, App, HttpResponse, HttpServer, Responder, web::Data};
mod api; 
mod models;
mod repository;
use repository::mongodb_repos::MongoRepo;
use api::files;
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
          .service(files::upload_file)
          .service(files::upload_file_path)
          .service(files::upload_file_dir)
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
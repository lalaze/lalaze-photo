use actix_web::{get, App, HttpResponse, HttpServer, Responder, web::Data, http};
mod api; 
mod models;
mod repository;
use repository::mongodb_repos::MongoRepo;
use actix_cors::Cors;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello from rust and mongoDB")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let db = MongoRepo::init().await;
  let db_data = Data::new(db);
  HttpServer::new(move || {
      let cors = Cors::default()
        .allowed_origin("*")
        .allowed_origin_fn(|origin, _req_head| {
            origin.as_bytes().ends_with(b".rust-lang.org")
        })
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600);
      App::new()
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
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
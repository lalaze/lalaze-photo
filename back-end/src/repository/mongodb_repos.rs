use std::env;
extern crate dotenv;
use dotenv::dotenv;
use actix_web::{web};
use futures_util::io::AsyncWriteExt;
use futures_util::StreamExt as _;
use mongodb::{
    bson::{extjson::de::Error},
    Client,
    Database,
    gridfs::{GridFsBucket}
};

pub struct MongoRepo {
   db: Database,
   bucket: GridFsBucket
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let database_name = match env::var("DATABASE") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let db = client.database(&database_name);
        let bucket = db.gridfs_bucket(None);
        MongoRepo { db, bucket }
    }

    pub async fn uploadFile(&self, file_name: String, data: web::BytesMut) -> Result<(), Error> {
        let mut upload_stream = &mut self.bucket.open_upload_stream(file_name, None);
        upload_stream.write_all(&data[..]).await.unwrap();
        upload_stream.close().await.unwrap();
        Ok(())
    }
}
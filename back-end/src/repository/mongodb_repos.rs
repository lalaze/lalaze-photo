use std::env;
extern crate dotenv;
use dotenv::dotenv;
use actix_web::{web};
use std::future::Future;
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

    pub async fn uploadFile(&self, file_name: String, mut payload: web::Payload) -> Result<(), Error> {
        let mut bytes = web::BytesMut::new();
        while let Some(item) = payload.next().await {
            bytes.extend_from_slice(&item.unwrap());
        }

        Ok(())
    }
}
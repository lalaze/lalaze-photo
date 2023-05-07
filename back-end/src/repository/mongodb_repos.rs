use std::env;
extern crate dotenv;
use dotenv::dotenv;
use futures_util::TryStreamExt;
use futures_util::io::AsyncWriteExt;
use crate::models::{ photo::Photo, tag::Tag };
use std::{fs};
use std::path::PathBuf;
use mongodb::{
    bson::{extjson::de::Error, doc},
    Client,
    Database,
    gridfs::{GridFsBucket}, Collection,
    options::FindOptions,
    results::{ UpdateResult, DeleteResult }
};

pub struct MongoRepo {
  db: Database,
  bucket: GridFsBucket,
  col: Collection<Photo>,
  col2: Collection<Tag>
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
        println!("{}", "数据库初始化");
        let db = client.database(&database_name);
        let bucket = db.gridfs_bucket(None);
        let col: Collection<Photo> = db.collection("photo");
        let col2: Collection<Tag> = db.collection("tags");
        println!("{}", "数据库初始化成功");
        MongoRepo { db, bucket, col, col2 }
    }

    pub async fn upload_file(&self, file_name: String, data: Vec<u8>) -> Result<(), Error> {
      let mut upload_stream = &mut self.bucket.open_upload_stream(file_name, None);
      upload_stream.write_all(&data[..]).await.unwrap();
      upload_stream.close().await.unwrap();
      Ok(())
    }

    pub async fn crate_photo(&self, file_name: String, location: String) -> Result<(), Error> {
      let filter = doc! {"_id": location.clone()};
      let result = self.col.count_documents(filter, None).await.unwrap();

      if result > 0 {
        return Ok(())
      }

      let photo = Photo {
        id: Some(location.clone()),
        name: file_name,
        location: location,
        tag: Vec::new()
      };

      match self.col.insert_one(photo, None).await {
        Ok(_) => println!("Insert successful"),
        Err(e) => {
            eprintln!("Insert error: {}", e);
        },
      }
      Ok(())
    }

    pub async fn crate_photo_dir(&self, path: &str) -> Result<(), Error> {
      let mut stack = vec![PathBuf::from(path)];

      loop {
        if let Some(dir) = stack.pop() {
            for entry in fs::read_dir(dir).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
    
                print!("{:?}", path);
                if path.is_dir() {
                    stack.push(path);
                } else {
                    self.crate_photo(path.clone().into_os_string().into_string().unwrap(),
                        path.clone().into_os_string().into_string().unwrap()).await.expect("create field");
                  }
              }
          } else {
              break;
          }
      }

      Ok(())
    }

    pub async fn get_photos(&self, offset: u64, limit: i64) -> Result<Vec<Photo>, Error> {

      let mut find_options = FindOptions::default();
      find_options.skip = Some(offset);
      find_options.limit = Some(limit);

      let mut cursor = self.col.find(None, find_options).await.unwrap();

      let mut photos: Vec<Photo> = Vec::new();

      while let Some(Photo) = cursor.try_next().await.ok().expect("Error mapping through cursor") {
        photos.push(Photo)
      }

      Ok(photos)
    }

    pub async fn get_photo(&self, id: &String) -> Result<Photo, Error> {
      let filter = doc! {"_id": id};
      let photo_detail = self
                .col
                .find_one(filter, None)
                .await
                .ok()
                .expect("Error getting user's detail");
      Ok(photo_detail.unwrap())
    }

    pub async fn edit_photos(&self, id: &String, new_photo: Photo) -> Result<UpdateResult, Error> {
      let filter = doc! {"_id": id};
      // pub name: String,
      // pub location: String,
      // pub tag: Option<String>,
      let new_doc = doc! {
        "$set":
            {
                "name": new_photo.name,
                "location": new_photo.location,
                "tag": new_photo.tag
            },
        };
      let updated_doc = self
        .col
        .update_one(filter, new_doc, None)
        .await
        .ok()
        .expect("Error updating photo");
      Ok(updated_doc)
    }

    pub async fn delete_photo(&self, id: &String) -> Result<DeleteResult, Error> {
      let filter = doc! {"_id": id};
      let photo_detail = self
          .col
          .delete_one(filter, None)
          .await
          .ok()
          .expect("Error deleting user");
      Ok(photo_detail)
    }

    pub async fn add_tag(&self, id: &String)  -> Result<DeleteResult, Error> {
      let filter = doc! {"_id": location.clone()};
      let result = self.col.count_documents(filter, None).await.unwrap();
    }
}
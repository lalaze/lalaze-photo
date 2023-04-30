use std::env;
extern crate dotenv;
use dotenv::dotenv;
use futures_util::io::AsyncWriteExt;
use crate::models::photo::Photo;
use std::{fs, path::Path};
use std::path::PathBuf;
use mongodb::{
    bson::{extjson::de::Error},
    Client,
    Database,
    gridfs::{GridFsBucket}, Collection, results::InsertOneResult
};

pub struct MongoRepo {
  db: Database,
  bucket: GridFsBucket,
  col: Collection<Photo>
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
        println!("{}", "数据库初始化成功");
        MongoRepo { db, bucket, col }
    }

    pub async fn upload_file(&self, file_name: String, data: Vec<u8>) -> Result<(), Error> {
      let mut upload_stream = &mut self.bucket.open_upload_stream(file_name, None);
      upload_stream.write_all(&data[..]).await.unwrap();
      upload_stream.close().await.unwrap();
      Ok(())
    }

    pub async fn crate_photo(&self, file_name: String, location: String) -> Result<(), Error> {
      let photo = Photo {
        id: Some(location.clone()),
        name: file_name,
        location: location,
        tag: Some("".to_string())
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

      // while let Some(dir) = stack.pop() {
      //   for entry in fs::read_dir(dir).unwrap() {
      //       let entry = entry.unwrap();
      //       let path = entry.path();

      //       print!("{:?}", path);
      //       if path.is_dir() {
      //         stack.push(path);
      //       } else {
      //         self.crate_photo(path.clone().into_os_string().into_string().unwrap(),
      //           path.clone().into_os_string().into_string().unwrap()).await.expect("create field");
      //       }
      //   }
      // }

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
}
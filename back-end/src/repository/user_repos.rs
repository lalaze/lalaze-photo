use crate::repository::mongodb_repos::MongoRepo;
use mongodb::{
  bson::{extjson::de::Error, doc},
  results::{ UpdateResult, DeleteResult },
  options::FindOptions,
};
use futures_util::TryStreamExt;
use crate::models::{ user::User };

impl MongoRepo {
  pub async fn create_user(&self, name: &String, password: &String) -> Result<(), Error> {
    let mut hasher = md5::Context::new();
    hasher.consume(password.as_bytes());
    // 计算 MD5 哈希值
    let result = hasher.compute();
    let hashed_password = format!("{:x}", result);

    let user = User {
      userName: name.to_string(),
      password: hashed_password
    };

    match self.col3.insert_one(user, None).await {
      Ok(_) => println!("Insert successful"),
      Err(e) => {
          eprintln!("Insert error: {}", e);
      },
    }
    Ok(())
  }

  pub async fn get_user(&self, username: &String) -> Result<Option<User>, Error>  {
    let filter = doc! {"_id": username};
    let result = self.col3.find_one(filter, None).await.unwrap();
    if let Some(doc) = result {
      Ok(Some(doc))
    } else {
      Ok(None)
    }
  }


  pub async fn edit_user(&self, name: &String, password: &String) -> Result<UpdateResult, Error> {
    let filter = doc! {"_id": name};
    let new_doc = doc! {
      "$set":
          {
            "password": password
          },
      };
    let updated_doc = self
      .col3
      .update_one(filter, new_doc, None)
      .await
      .ok()
      .expect("Error updating photo");
    Ok(updated_doc)
  }
}
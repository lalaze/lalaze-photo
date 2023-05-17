use crate::repository::mongodb_repos::MongoRepo;
use mongodb::{
  bson::{extjson::de::Error, doc},
  results::{ UpdateResult, DeleteResult }
};
use crate::models::{ tag::Tag };
use mongodb::bson::oid::ObjectId;

impl MongoRepo {
  pub async fn add_tag(&self, name: &String, color: &String)  -> Result<(), Error> {
    let tag = Tag {
      name: name.to_string(),
      color: color.to_string()
    };
  
    match self.col2.insert_one(tag, None).await {
      Ok(_) => println!("Insert successful"),
      Err(e) => {
          eprintln!("Insert error: {}", e);
      },
    }
    Ok(())
  }

  pub async fn edit_tag(&self, id: &String, name: &String, color: &String) -> Result<UpdateResult, Error> {
    let filter = doc! {"_id": id};
    let new_doc = doc! {
      "$set":
          {
            "name": name,
            "color": color
          },
      };
    let updated_doc = self
      .col2
      .update_one(filter, new_doc, None)
      .await
      .ok()
      .expect("Error updating photo");
    Ok(updated_doc)
  }

  pub async fn delete_tag(&self, id: &String) -> Result<DeleteResult, Error> {
    let filter = doc! {"_id": ObjectId::parse_str(id).unwrap()};

    let photo_detail = self
        .col2
        .delete_one(filter, None)
        .await
        .ok()
        .expect("Error deleting tag");
    Ok(photo_detail)
  }
}

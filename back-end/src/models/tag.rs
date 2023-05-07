use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
  #[serde(rename = "_id")]
  pub id: i64,
  pub name: String,
}
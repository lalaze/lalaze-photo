use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
  #[serde(rename = "_id")]
  pub userName: String,
  pub password: String,
}
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MyResponse<T> {
  pub result: String,
  pub message: String,
  pub data: Option<T>,
}
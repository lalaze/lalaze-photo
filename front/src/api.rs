use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use reqwest::header::{ HeaderMap, CONTENT_TYPE };

const URL: &str = "http://localhost/8083/login";

#[derive(Serialize, Deserialize)]
struct LoginReq {
  username: String,
  password: String
}

#[tokio::main]
pub async fn login(name: &String, password: &String) ->  Result<(), Box<dyn std::error::Error>>  {
  let client = reqwest::Client::new();
  
  let body = LoginReq {
    username: name.to_string(),
    password: password.to_string()
  };

  let json_str = serde_json::to_string(&body).unwrap();

  let resp = client.post("https://api.notion.com/v1/pages")
        .header(CONTENT_TYPE, "application/json")
        .body(json_str)
        .send()
        .await?;

  let json_value = resp.json::<HashMap<String, String>>().await?;
  
  Ok(())
}
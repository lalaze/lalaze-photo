use gloo::{ net::http::Request, net::http::Response, console};
use serde::{Deserialize, Serialize};
use yew::{Callback};
use crate::message;
use serde_json::Value;

const URL: &str = "http://127.0.0.1:8083";

#[derive(Serialize, Deserialize)]
pub struct Resp {
  pub result: String,
  pub message: String,
  pub data: Option<Value>,
}

#[derive(Serialize, Deserialize)]
struct LoginReq {
  username: String,
  password: String
}

pub async fn login(name: &str, password: &str) ->  Result<Option<Resp>, Box<dyn std::error::Error>>  {

  let body = LoginReq {
    username: name.to_string(),
    password: password.to_string()
  };

  let json_str = serde_json::to_string(&body).unwrap();

  let resp = Request::post(&format!("{}{}", URL, "/login")).body(json_str).header("Content-Type", "application/json")
    .send()
    .await
    .unwrap();
  Ok(Some(resp.json::<Resp>().await.unwrap()))
  // if resp.status() == 200 {
  //   // let res = resp.json::<Resp>().await.unwrap();
  //   // console::log!(res.result.as_str());
  //   // match res.result.as_str() {
  //   //   "0" => Ok(Some(resp.json::<Resp>().await.unwrap())),
  //   //   _ => {
  //   //     console::log!("here");
  //   //     message::Msg::SpawnCounterAppInstance(message::Message {
  //   //       message_type: message::MessageType::Danger,
  //   //       text: res.message,
  //   //       long: None,
  //   //     });
  //   //     Ok(None)
  //   //   }
  //   // }
  //   Ok(Some(resp.json::<Resp>().await.unwrap()))
  // } else {
  //   message::Msg::SpawnCounterAppInstance(message::Message {
  //     message_type: message::MessageType::Danger,
  //     text: format!("{} {}", "error", resp.status()),
  //     long: None,
  //   });
  //   Ok(None)
  // }
}
use gloo_net::{ http::Request, http::Response};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {

    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

const URL: &str = "http://localhost/8083";

#[derive(Serialize, Deserialize)]
struct LoginReq {
  username: String,
  password: String
}

pub async fn login(name: &str, password: &str) ->  Result<(), Box<dyn std::error::Error>>  {
  log("456");

  let body = LoginReq {
    username: name.to_string(),
    password: password.to_string()
  };

  let json_str = serde_json::to_string(&body).unwrap();

  let resp = Request::post(&format!("{} {}", URL, "/login")).body(json_str)
    .send()
    .await
    .unwrap();

  print!("{}", resp.text().await.unwrap());

  Ok(())
}
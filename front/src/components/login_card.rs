use yew::prelude::*;
use web_sys::{EventTarget, HtmlInputElement};
use crate::message::{Msg, MessageType, Message};
use crate::api::auth::login;
use wasm_bindgen_futures::spawn_local;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub message: Callback<Message>,
}   

#[function_component]
pub fn login_card(props: &Props) -> Html {
  let username = use_state(|| String::new());
  let username_node = use_node_ref();
  let on_username_change = {
    let input_node_ref = username_node.clone();
    let username = username.clone();
    Callback::from(move |_| {
      if let Some(input) = input_node_ref.cast::<HtmlInputElement>() {
          let value = input.value();
          username.set(value)
      }
    })
  };

  let password = use_state(|| String::new());
  let password_node = use_node_ref();
  let on_password_change ={
    let input_node_ref = password_node.clone();
    let password = password.clone();
    Callback::from(move |_| {
      if let Some(input) = input_node_ref.cast::<HtmlInputElement>() {
          let value = input.value();
          password.set(value)
      }
    })
  };

  let do_this_func = props.message.clone();

  let handle_click = {
    Callback::from(move |_| {
      let username = username.clone();
      let password = password.clone();
      spawn_local(async move {
        let res = login(username.as_str(), password.as_str()).await.unwrap();
        if res.result == "0" {
          do_this_func.emit(Message {
            message_type: MessageType::Success,
            text: String::from("Login success"),
            long: None,
          });
        }
      })
    })
  };

  html! {
    <div class={classes!(String::from("box is-dark login-box"))}>
      <div class={classes!(String::from("field"))}>
        <div class={classes!(String::from("label"))}>{ "User Name" }</div>
        <div class={classes!(String::from("control"))}>
          <input ref={username_node}  class={classes!(String::from("input is-primary"))} type="text" onchange={on_username_change} />
        </div>
      </div>
      <div class={classes!(String::from("field"))}>
        <div class={classes!(String::from("label"))}>{ "Password" }</div>
        <div class={classes!(String::from("control"))}>
          <input ref={password_node} class={classes!( String::from("input is-primary"))} type="password"  onchange={on_password_change} />
        </div>
      </div>
      <button class={classes!(String::from("button is-primary"))} onclick={handle_click}>
        { "Sign in" }
      </button>
    </div>
}
}
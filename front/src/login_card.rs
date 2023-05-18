use yew::prelude::*;

#[function_component]
pub fn login_card() -> Html {
  let handle_click = Callback::from(move |_| {
  });

  html! {
    <div class={classes!(String::from("box is-dark login-box"))}>
      <div class={classes!(String::from("field"))}>
        <div class={classes!(String::from("label"))}>{ "User Name" }</div>
        <div class={classes!(String::from("control"))}>
          <input class={classes!(String::from("input is-primary"))} type="text" />
        </div>
      </div>
      <div class={classes!(String::from("field"))}>
        <div class={classes!(String::from("label"))}>{ "Password" }</div>
        <div class={classes!(String::from("control"))}>
          <input class={classes!( String::from("input is-primary"))} type="password" />
        </div>
      </div>

      <button class={classes!(String::from("button is-primary"))} onclick={handle_click}>
        { "Sign in" }
      </button>
    </div>
}
}
use yew::prelude::*;
mod login_card;
mod api;
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

#[function_component]
fn App() -> Html {
    html! {
        <div class={classes!(String::from("content"))}>
            <login_card::login_card />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
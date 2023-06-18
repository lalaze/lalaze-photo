use gloo::console;
use slab::Slab;
use web_sys::Element;
use yew::prelude::*;
pub mod api;
pub mod components;
pub mod utils;

use components::message;

pub struct App {
    apps: Slab<(Element, AppHandle<message::MessageModel>)>,
    apps_container_ref: NodeRef,
}

impl Component for App {
    type Message = message::Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            apps: Slab::new(),
            apps_container_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            message::Msg::SpawnCounterAppInstance(p) => self.get_message(ctx, p),
            message::Msg::DestroyCounterApp(app_id) => {
                // Get the app from the app slabmap
                let (app_div, app) = self.apps.remove(app_id);

                // Destroy the app
                app.destroy();

                // Remove the app div from the DOM
                app_div.remove()
            }
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        utils::set_zoom();

        let message: Callback<message::Message> = Callback::from(move |p: message::Message| {
          message::Msg::SpawnCounterAppInstance(p);
        });

        html! {
          <div class={classes!("app")} ref={self.apps_container_ref.clone()}>
            <div class={classes!(String::from("content"))}>
              <components::login_card::login_card {message} />
            </div>
          </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

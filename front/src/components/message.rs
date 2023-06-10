use crate::api::auth::Resp;
use crate::App;
use gloo::console;
use gloo::timers::future::TimeoutFuture;
use gloo::utils::document;
use wasm_bindgen_futures::spawn_local;
use web_sys::Element;
use yew::prelude::*;

#[derive(Clone, Debug)]
pub enum MessageType {
    Success,
    Danger,
}

impl PartialEq for MessageType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (MessageType::Success, MessageType::Success)
            | (MessageType::Danger, MessageType::Danger) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Message {
    pub message_type: MessageType,
    pub text: String,
    pub long: Option<u32>,
}

pub struct MessageModel {
    apps_container_ref: NodeRef,
}

#[derive(Clone, Properties, PartialEq)]
pub struct MessageProps {
    pub destroy_callback: Callback<()>,
    pub message: Message,
}

impl Component for MessageModel {
    type Message = ();
    type Properties = MessageProps;

    fn create(ctx: &Context<Self>) -> Self {
        let destroy_callback = ctx.props().destroy_callback.clone();
        spawn_local(async move {
            TimeoutFuture::new(3_000).await;
            destroy_callback.emit(());
        });
        Self {
            apps_container_ref: NodeRef::default(),
        }
    }

    // fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    // }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let message_type = match ctx.props().message.message_type {
            MessageType::Success => "is-success",
            MessageType::Danger => "is-danger",
        };

        html! {
          <>
            <article class={classes!("message", "message-location", message_type)}>
              <div class="message-body">
                { ctx.props().message.text.clone() }
              </div>
            </article>
          </>
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        console::log!("CounterModel app destroyed");
    }
}

pub enum Msg {
    SpawnCounterAppInstance(Message),
    DestroyCounterApp(usize),
}

impl App {
    pub fn handle_message(&mut self, ctx: &Context<Self>, resp: Resp) {
        match resp.result.as_str() {
            "0" => {}
            _ => {
                console::log!("here");
                self.get_message(
                    ctx,
                    Message {
                        message_type: MessageType::Danger,
                        text: resp.message,
                        long: None,
                    },
                );
            }
        }
    }

    pub fn get_message(&mut self, ctx: &Context<Self>, p: Message) {
        console::log!("SpawnCounterAppInstance");
        let app_container = self
            .apps_container_ref
            .cast::<Element>()
            .expect("Failed to cast app container div to HTMLElement");

        let app_div = document()
            .create_element("div")
            .expect("Failed to create <div> element");

        let _ = app_container
            .append_child(&app_div)
            .expect("Failed to append app div app container div");

        let app_entry = self.apps.vacant_entry();

        let app_key = app_entry.key();
        let new_counter_app = yew::Renderer::<MessageModel>::with_root_and_props(
            app_div.clone(),
            MessageProps {
                message: p,
                destroy_callback: ctx
                    .link()
                    .callback(move |_| Msg::DestroyCounterApp(app_key)),
            },
        )
        .render();

        // Insert the app and the app div to our app collection
        app_entry.insert((app_div, new_counter_app));
    }
}

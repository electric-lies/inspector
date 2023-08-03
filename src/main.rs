
use yew::html::Scope;

use yew::prelude::*;

// use log::info;

mod text_input;
use crate::text_input::TextInput;

pub enum Msg {
    LenChanged(i32),
}

pub struct App {
    input_len: i32,
}
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { input_len: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::LenChanged(val) => self.input_len = val,
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
            { self.view_input(ctx.link()) }
            </div>
        }
    }
}

impl App {
    fn view_input(&self, link: &Scope<Self>) -> Html {
        let on_change = link.batch_callback(|input: String| {
            Some(Msg::LenChanged(input.len() as i32))
        });
        html! {
            <div>
            <TextInput {on_change} desc={"text here: "}/>
                <p>{self.input_len}</p>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    // Logging
    // info!("Some info");
    // log::error!("Error message");
    yew::Renderer::<App>::new().render();
}

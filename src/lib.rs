extern crate stdweb;
#[macro_use]
extern crate yew;

mod byte_boxes;

use yew::prelude::*;
use yew::services::ConsoleService;
use byte_boxes::ByteBoxes;

pub struct Model {
    console: ConsoleService,
    value: u8,
}

pub enum Msg {
    Increment,
    Decrement,
    SetValue(u8),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            console: ConsoleService::new(),
            value: 42,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.value = self.value + 1;
                self.console.log("plus one");
            }
            Msg::Decrement => {
                self.value = self.value - 1;
                self.console.log("minus one");
            }
            Msg::SetValue(value) => {
                self.value = value;
                self.console.log("value set to...");
                self.console.log(&value.to_string());
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <span>{"Value: "}{ self.value }</span>
                <br />
                <ByteBoxes: value=self.value, onchange=Msg::SetValue, />
                <div class="menu",>
                    <button onclick=|_| Msg::Increment,>{ "Increment" }</button>
                    <button onclick=|_| Msg::Decrement,>{ "Decrement" }</button>
                    <button onclick=|_| Msg::SetValue(0),>{ "Clear" }</button>
                </div>
            </div>
        }
    }
}


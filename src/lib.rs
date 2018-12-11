extern crate stdweb;
#[macro_use]
extern crate yew;

mod byte_boxes;

use yew::prelude::*;
use yew::services::ConsoleService;
use byte_boxes::ByteBoxes;
use std::str;

pub struct Model {
    console: ConsoleService,
    length: u8,
    bytes: Vec<u8>,
}

pub enum Msg {
    Increment,
    Decrement,
    SetLength(u8),
    SetValueAt(usize, u8),
}

impl Model {
    fn update_length(&mut self) {
        while self.bytes.len() < self.length as usize {
            self.bytes.push(0);
        }
        while self.bytes.len() > self.length as usize {
            self.bytes.pop();
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            console: ConsoleService::new(),
            length: 0,
            bytes: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.length = self.length + 1;
                self.console.log("plus one");
                self.update_length();
            }
            Msg::Decrement => {
                self.length = self.length - 1;
                self.console.log("minus one");
                self.update_length();
            }
            Msg::SetLength(value) => {
                self.length = value;
                self.console.log("value set to...");
                self.console.log(&value.to_string());
                self.update_length();
            }
            Msg::SetValueAt(i, value) => {
                self.bytes[i] = value;
                self.console.log("value set to...");
                self.console.log(&value.to_string());
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let byte_input = |i| html! {
            <span>
                <ByteBoxes: value=self.bytes[i], onchange=move |b| Msg::SetValueAt(i, b),/>
            </span>
        };
        let byte_value = |b| { html! {
            <span>{ b }{" "}</span>
        }};
        html! {
            <div>
                <span>{"Length: "}{ self.length }</span>
                <br />
                <span>
                    {"Values: "}
                    { for (self.bytes.iter()).map(byte_value) }
                </span>
                <br />
                <span>
                    { "String:" }
                    { str::from_utf8(&self.bytes).unwrap_or("") }
                </span>
                <br />
                <ByteBoxes: value=self.length, onchange=Msg::SetLength, />
                <div>
                    { for (0..self.bytes.len()).map(byte_input) }
                </div>
                <div class="menu",>
                    <button onclick=|_| Msg::Increment,>{ "Increment" }</button>
                    <button onclick=|_| Msg::Decrement,>{ "Decrement" }</button>
                    <button onclick=|_| Msg::SetLength(0),>{ "Clear" }</button>
                </div>
            </div>
        }
    }
}


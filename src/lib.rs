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
    SetLength(u8),
    SetValueAt(usize, u8),
    SetStringValue(String),
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
            Msg::SetStringValue(str_val) => {
                self.console.log("String value...");
                self.console.log(&str_val);
                self.length = str_val.len() as u8;
                self.bytes = str_val.into_bytes();
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let byte_input = |i| html! {
            <span>
                { "|" }
                <ByteBoxes: value=self.bytes[i], onchange=move |b| Msg::SetValueAt(i, b),/>
            </span>
        };
        html! {
            <div>
                <dic>
                    <input
                        oninput=|e| Msg::SetStringValue(e.value),
                        value= str::from_utf8(&self.bytes).unwrap_or(""), />
                </dic>
                <div>
                    <ByteBoxes: value=self.length, onchange=Msg::SetLength, />
                    { for (0..self.bytes.len()).map(byte_input) }
                </div>
            </div>
        }
    }
}


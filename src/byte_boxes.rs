use yew::prelude::*;

pub struct ByteBoxes {
    bits: [bool; 8],
    onchange: Option<Callback<u8>>,
}

pub enum Msg {
    ToggleBit(usize),
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub value: u8,
    pub onchange: Option<Callback<u8>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            value: 0,
            onchange: None,
        }
    }
}

impl ByteBoxes {
    fn get_value(&self) -> u8 {
        let mut value = 0;
        for i in 0..8 {
            if self.bits[i] {
                value |= 1 << i;
            }
        }
        value
    }
    
    fn set_value(&mut self, value: u8) {
        for i in 0..8 {
            self.bits[i] = (value & (1 << i)) > 0;
        }
    }
}

impl Component for ByteBoxes {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        let mut byte_boxes = ByteBoxes {
            bits: [false; 8],
            onchange: props.onchange,
        };
        byte_boxes.set_value(props.value);
        byte_boxes
    }
    
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.set_value(props.value);
        self.onchange = props.onchange;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleBit(i) => {
                self.bits[i] = !self.bits[i];
                if let Some(ref onchange) = self.onchange {
                    onchange.emit(self.get_value());
                }
            }
        }
        true
    }
}

impl Renderable<ByteBoxes> for ByteBoxes {
    fn view(&self) -> Html<Self> {
        let bit_box = |x| html! {
            <input
                type="checkbox",
                checked=self.bits[x],
                onclick=|_| Msg::ToggleBit(x),
            />
        };
        html! { <span>{ for (0..8).rev().map(bit_box) }</span> }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_a_value() {
        let byte_boxes = ByteBoxes {
            bits: [false, true, false, true, false, true, false, false],
            onchange: None,
        };
        assert_eq!(byte_boxes.get_value(), 42);
    }
    
    #[test]
    fn it_sets_a_value() {
        let mut byte_boxes = ByteBoxes {
            bits: [false, false, false, false, false, false, false, false],
            onchange: None,
        };
        byte_boxes.set_value(42);
        assert_eq!(byte_boxes.bits,
                [false, true, false, true, false, true, false, false]);
    }
}

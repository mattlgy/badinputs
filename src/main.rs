extern crate yew;
extern crate badinputs;

use yew::prelude::*;
use badinputs::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}

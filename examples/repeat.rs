use dioxus::prelude::*;
use dioxus_type_animation::{Repeat, SequenceElement, TypeAnimation};

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        TypeAnimation {
            sequence: vec![SequenceElement::from("Loop me"), SequenceElement::from(1000_u64)],
            repeat: Repeat::Count(3),
        }
    }
}

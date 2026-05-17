use dioxus::prelude::*;
use dioxus_type_animation::{SequenceElement, TypeAnimation};

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        TypeAnimation {
            sequence: vec![SequenceElement::from("No cursor")],
            cursor: false,
        }
    }
}

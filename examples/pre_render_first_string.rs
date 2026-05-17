use dioxus::prelude::*;
use dioxus_type_animation::{SequenceElement, TypeAnimation};

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        TypeAnimation {
            sequence: vec![
                SequenceElement::from("Already visible"),
                SequenceElement::from(1000_u64),
                SequenceElement::from("Then animated"),
            ],
            pre_render_first_string: true,
        }
    }
}

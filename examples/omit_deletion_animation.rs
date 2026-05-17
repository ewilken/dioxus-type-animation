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
                SequenceElement::from("Dioxus is fun"),
                SequenceElement::from(1000_u64),
                SequenceElement::from("Dioxus is fast"),
            ],
            omit_deletion_animation: true,
        }
    }
}

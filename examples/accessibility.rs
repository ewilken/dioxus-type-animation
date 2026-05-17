use dioxus::prelude::*;
use dioxus_type_animation::{SequenceElement, TypeAnimation};

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        TypeAnimation {
            sequence: vec![SequenceElement::from("Fast-changing animated text")],
            aria_label: Some("Animated product tagline".to_string()),
            role: Some("text".to_string()),
        }
    }
}

use dioxus::prelude::*;
use dioxus_type_animation::{SequenceElement, TypeAnimation, Wrapper};

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        TypeAnimation {
            wrapper: Wrapper::H1,
            sequence: vec![SequenceElement::from("Animated heading")],
        }
    }
}

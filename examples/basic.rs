use dioxus::prelude::*;
use dioxus_type_animation::{Repeat, SequenceElement, Speed, TypeAnimation, Wrapper};

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        TypeAnimation {
            sequence: vec![
                SequenceElement::from("We produce food for Mice"),
                SequenceElement::from(1000_u64),
                SequenceElement::from("We produce food for Hamsters"),
                SequenceElement::from(1000_u64),
                SequenceElement::from("We produce food for Guinea Pigs"),
                SequenceElement::from(1000_u64),
                SequenceElement::from("We produce food for Chinchillas"),
                SequenceElement::from(1000_u64),
            ],
            wrapper: Wrapper::Span,
            speed: Speed::Preset(50),
            style: Some("font-size: 2em; display: inline-block;".to_string()),
            repeat: Repeat::Infinite,
        }
    }
}

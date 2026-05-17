use dioxus::prelude::*;
use dioxus_type_animation::{SequenceElement, Speed, TypeAnimation};

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        TypeAnimation {
            sequence: vec![
                SequenceElement::from("Type quickly"),
                SequenceElement::from(500_u64),
                SequenceElement::from("Delete slowly"),
            ],
            speed: Speed::Preset(80),
            deletion_speed: Some(Speed::KeyStrokeDelayInMs(120)),
        }
    }
}

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
                SequenceElement::from("Typing..."),
                SequenceElement::from(750_u64),
                SequenceElement::from(|| println!("Done typing")),
                SequenceElement::from("Finished."),
            ],
        }
    }
}

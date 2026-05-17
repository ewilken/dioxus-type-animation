use dioxus::prelude::*;
use dioxus_type_animation::{SequenceElement, StringSplitter, TypeAnimation};
use std::rc::Rc;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let splitter: StringSplitter = Rc::new(|text: &str| {
        // Replace this with unicode-segmentation or another grapheme splitter
        // if your application needs full grapheme-cluster support.
        text.chars().map(|char| char.to_string()).collect()
    });

    rsx! {
        TypeAnimation {
            sequence: vec![SequenceElement::from("👨‍👩‍👧‍👦 family")],
            splitter: Some(splitter),
        }
    }
}

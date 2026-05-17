use dioxus::prelude::*;
use dioxus_type_animation::{
    Repeat, SequenceElement, Speed, StringSplitter, TypeAnimation, Wrapper,
};
use std::rc::Rc;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let splitter: StringSplitter =
        Rc::new(|text: &str| text.chars().map(|char| char.to_string()).collect());

    rsx! {
        TypeAnimation {
            sequence: vec![
                SequenceElement::from("Build apps with React-style ergonomics"),
                SequenceElement::from(1200_u64),
                SequenceElement::from("Build apps with Rust safety"),
                SequenceElement::from(1200_u64),
                SequenceElement::from(|| println!("Finished one sequence pass")),
            ],
            repeat: Repeat::Infinite,
            wrapper: Wrapper::H2,
            speed: Speed::Preset(55),
            deletion_speed: Some(Speed::KeyStrokeDelayInMs(35)),
            omit_deletion_animation: false,
            cursor: true,
            pre_render_first_string: false,
            splitter: Some(splitter),
            class: Some("type-animation-heading".to_string()),
            style: Some("display: inline-block; color: #4f46e5;".to_string()),
            aria_label: Some("Dioxus type animation example".to_string()),
            role: Some("text".to_string()),
        }
    }
}

//! Dioxus port of the `react-type-animation` typewriter component.
//!
//! The main entry point is [`TypeAnimation`]. It mirrors the React library's
//! feature set with Dioxus-friendly Rust types: a sequence of text, delays, and
//! callbacks; configurable speed/deletion speed; finite or infinite repetition;
//! wrapper element selection; optional cursor CSS; and custom text splitting.
//!
//! # Example
//!
//! ```rust,no_run
//! use dioxus::prelude::*;
//! use dioxus_type_animation::{Repeat, SequenceElement, Speed, TypeAnimation};
//!
//! fn App() -> Element {
//!     rsx! {
//!         TypeAnimation {
//!             sequence: vec![
//!                 SequenceElement::from("We produce food for Mice"),
//!                 SequenceElement::from(1000_u64),
//!                 SequenceElement::from("We produce food for Hamsters"),
//!                 SequenceElement::from(1000_u64),
//!             ],
//!             speed: Speed::Preset(50),
//!             repeat: Repeat::Infinite,
//!             style: Some("font-size: 2em; display: inline-block;".to_string()),
//!         }
//!     }
//! }
//! ```

#![allow(non_snake_case)]

mod animation;
mod repeat;
mod sequence;
mod speed;
mod type_animation;
mod wrapper;

pub use repeat::Repeat;
pub use sequence::{SequenceCallback, SequenceElement, StringSplitter};
pub use speed::Speed;
pub use type_animation::{TypeAnimation, TypeAnimationProps};
pub use wrapper::Wrapper;

pub(crate) const CURSOR_CLASS: &str = "dioxus-type-animation__type";

/// CSS used by [`TypeAnimation`] when `cursor` is enabled.
///
/// The component injects this stylesheet automatically. It is also exported so
/// applications can include it globally if they prefer.
pub const CURSOR_CSS: &str = r#"
.dioxus-type-animation__type::after {
  content: '|';
  animation: dioxus-type-animation__cursor 1.1s infinite step-start;
}

@keyframes dioxus-type-animation__cursor {
  50% {
    opacity: 0;
  }
}
"#;

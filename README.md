# dioxus-type-animation

[![CI](https://github.com/ewilken/dioxus-type-animation/workflows/CI/badge.svg)](https://github.com/ewilken/dioxus-type-animation/actions?query=workflow%3ACI)
[![crates.io](https://img.shields.io/crates/v/dioxus-type-animation.svg)](https://crates.io/crates/dioxus-type-animation)
[![docs.rs](https://docs.rs/dioxus-type-animation/badge.svg)](https://docs.rs/dioxus-type-animation)
[![license: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/ewilken/dioxus-type-animation/blob/main/LICENSE)

A customizable Dioxus typewriter animation component inspired by [`react-type-animation`](https://github.com/maxeth/react-type-animation).

This crate provides a `TypeAnimation` component with a Rust-friendly API for animated typing/deleting effects, delays, callbacks, repeat behavior, custom wrappers, cursor styling, and custom string splitting.

## Installation

Add the crate to your Dioxus project:

```toml
[dependencies]
dioxus = "0.7"
dioxus-type-animation = "0.1"
```

## Usage

### Basic usage

```rust
use dioxus::prelude::*;
use dioxus_type_animation::{Repeat, SequenceElement, Speed, TypeAnimation, Wrapper};

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
```

### Sequence items

The `sequence` prop accepts three kinds of items:

- `SequenceElement::Text(String)` / `SequenceElement::from("text")` — transition to this text.
- `SequenceElement::Delay(u64)` / `SequenceElement::from(1000_u64)` — wait for this many milliseconds.
- `SequenceElement::Callback(Rc<dyn Fn()>)` / `SequenceElement::from(|| { ... })` — run a callback.

```rust
use dioxus::prelude::*;
use dioxus_type_animation::{SequenceElement, TypeAnimation};

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
```

### Repeat behavior

By default, a sequence runs once. `Repeat::Count(n)` means the animation runs once plus `n` repeats, matching `react-type-animation` behavior. Use `Repeat::Infinite` to loop forever.

```rust
use dioxus::prelude::*;
use dioxus_type_animation::{Repeat, SequenceElement, TypeAnimation};

fn App() -> Element {
    rsx! {
        TypeAnimation {
            sequence: vec![SequenceElement::from("Loop me"), SequenceElement::from(1000_u64)],
            repeat: Repeat::Count(3),
        }
    }
}
```

### Speed options

`Speed::Preset(value)` mirrors the React library's `speed={value}` prop. Numeric speed values are normalized to `abs(value - 100)` milliseconds per keystroke, then randomized by ±50% for a natural typing rhythm.

Use `Speed::KeyStrokeDelayInMs(value)` for an exact base keystroke delay in milliseconds, equivalent to React's `{ type: "keyStrokeDelayInMs", value }` option.

```rust
use dioxus::prelude::*;
use dioxus_type_animation::{SequenceElement, Speed, TypeAnimation};

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
```

### Omit deletion animation

Set `omit_deletion_animation` to `true` to jump directly to the deletion end-state and only animate writing the new text.

```rust
use dioxus::prelude::*;
use dioxus_type_animation::{SequenceElement, TypeAnimation};

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
```

### Wrapper elements

The default wrapper is `Wrapper::Span`. You can choose from `p`, `div`, `span`, `strong`, `a`, `h1`-`h6`, and `b` via the `Wrapper` enum.

```rust
use dioxus::prelude::*;
use dioxus_type_animation::{SequenceElement, TypeAnimation, Wrapper};

fn App() -> Element {
    rsx! {
        TypeAnimation {
            wrapper: Wrapper::H1,
            sequence: vec![SequenceElement::from("Animated heading")],
        }
    }
}
```

### Cursor styling

The blinking cursor is enabled by default. The component automatically injects CSS equivalent to:

```rust
.dioxus-type-animation__type::after {
  content: '|';
  animation: dioxus-type-animation__cursor 1.1s infinite step-start;
}

@keyframes dioxus-type-animation__cursor {
  50% {
    opacity: 0;
  }
}
```

Disable it with `cursor: false`:

```rust
use dioxus::prelude::*;
use dioxus_type_animation::{SequenceElement, TypeAnimation};

fn App() -> Element {
    rsx! {
        TypeAnimation {
            sequence: vec![SequenceElement::from("No cursor")],
            cursor: false,
        }
    }
}
```

### Pre-render the first string

Set `pre_render_first_string` to `true` to render the first string immediately before animation starts.

```rust
use dioxus::prelude::*;
use dioxus_type_animation::{SequenceElement, TypeAnimation};

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
```

### Accessibility attributes

You can pass `aria_label`, `aria_hidden`, and `role`. When `aria_label` is set, the animated visual text is rendered inside an inner `aria-hidden="true"` span so assistive technology reads the stable label instead of every intermediate typing state.

```rust
use dioxus::prelude::*;
use dioxus_type_animation::{SequenceElement, TypeAnimation};

fn App() -> Element {
    rsx! {
        TypeAnimation {
            sequence: vec![SequenceElement::from("Fast-changing animated text")],
            aria_label: Some("Animated product tagline".to_string()),
            role: Some("text".to_string()),
        }
    }
}
```

### Custom splitter

The default splitter uses `text.chars()`, similar to JavaScript's `[...text]`. If you need grapheme-aware splitting for complex emoji or combined characters, provide your own splitter.

```rust
use dioxus::prelude::*;
use dioxus_type_animation::{SequenceElement, StringSplitter, TypeAnimation};
use std::rc::Rc;

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
```

## API reference

| Prop                      | Type                     | Default             | Description                                        |
| ------------------------- | ------------------------ | ------------------- | -------------------------------------------------- |
| `sequence`                | `Vec<SequenceElement>`   | required            | Animation sequence of text, delays, and callbacks. |
| `repeat`                  | `Repeat`                 | `Repeat::Count(0)`  | Number of repeats or infinite loop.                |
| `wrapper`                 | `Wrapper`                | `Wrapper::Span`     | HTML element used as wrapper.                      |
| `speed`                   | `Speed`                  | `Speed::Preset(40)` | Typing speed.                                      |
| `deletion_speed`          | `Option<Speed>`          | `None`              | Deletion speed. Falls back to `speed`.             |
| `omit_deletion_animation` | `bool`                   | `false`             | Skip animated deletion steps.                      |
| `cursor`                  | `bool`                   | `true`              | Show default blinking cursor.                      |
| `pre_render_first_string` | `bool`                   | `false`             | Render first string before animation starts.       |
| `splitter`                | `Option<StringSplitter>` | `None`              | Custom text splitting function.                    |
| `class`                   | `Option<String>`         | `None`              | Class applied to wrapper.                          |
| `style`                   | `Option<String>`         | `None`              | Inline style applied to wrapper.                   |
| `aria_label`              | `Option<String>`         | `None`              | Wrapper `aria-label`.                              |
| `aria_hidden`             | `Option<String>`         | `None`              | Wrapper `aria-hidden`.                             |
| `role`                    | `Option<String>`         | `None`              | Wrapper role.                                      |

## Notes

Like the React implementation, `TypeAnimation` is intentionally immutable: prop changes are treated as equal and do not restart the animation. If you need to restart with different props, mount a new component instance, for example by changing its Dioxus `key`.

Callbacks are Rust closures (`Rc<dyn Fn()>`) instead of React callbacks receiving an `HTMLElement | null`. The Dioxus implementation drives text with signals rather than directly mutating the DOM.

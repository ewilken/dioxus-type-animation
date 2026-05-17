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

use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use std::rc::Rc;

const DEFAULT_SPEED: u64 = 40;
const CURSOR_CLASS: &str = "dioxus-type-animation__type";

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

/// Function used to split a string into editable units.
///
/// The default splitter is Unicode scalar-value based (`text.chars()`), which is
/// the closest Rust equivalent to JavaScript's `[...text]`. For grapheme-aware
/// animation, pass a splitter backed by a crate such as `unicode-segmentation`.
pub type StringSplitter = Rc<dyn Fn(&str) -> Vec<String>>;

/// Callback entry in an animation sequence.
pub type SequenceCallback = Rc<dyn Fn()>;

/// A single animation sequence item.
#[derive(Clone)]
pub enum SequenceElement {
    /// Transition from the current text to this text.
    Text(String),
    /// Wait for the given number of milliseconds.
    Delay(u64),
    /// Invoke a callback.
    Callback(SequenceCallback),
}

impl From<&str> for SequenceElement {
    fn from(value: &str) -> Self {
        Self::Text(value.to_string())
    }
}

impl From<String> for SequenceElement {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<u64> for SequenceElement {
    fn from(value: u64) -> Self {
        Self::Delay(value)
    }
}

impl From<usize> for SequenceElement {
    fn from(value: usize) -> Self {
        Self::Delay(value as u64)
    }
}

impl<F> From<F> for SequenceElement
where
    F: Fn() + 'static,
{
    fn from(value: F) -> Self {
        Self::Callback(Rc::new(value))
    }
}

/// Speed configuration equivalent to the React library's `speed` and
/// `deletionSpeed` props.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Speed {
    /// React-compatible speed value, normally `1..=99`.
    ///
    /// Like `react-type-animation`, this is normalized to `abs(value - 100)`
    /// milliseconds per keystroke before random jitter is applied.
    Preset(u64),
    /// Direct keystroke delay in milliseconds, equivalent to
    /// `{ type: "keyStrokeDelayInMs", value }` in the React API.
    KeyStrokeDelayInMs(u64),
}

impl Default for Speed {
    fn default() -> Self {
        Self::Preset(DEFAULT_SPEED)
    }
}

impl From<u64> for Speed {
    fn from(value: u64) -> Self {
        Self::Preset(value)
    }
}

impl From<usize> for Speed {
    fn from(value: usize) -> Self {
        Self::Preset(value as u64)
    }
}

/// Repeat behavior for an animation sequence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Repeat {
    /// Play the sequence once plus `count` repeats.
    Count(usize),
    /// Loop forever.
    Infinite,
}

impl Default for Repeat {
    fn default() -> Self {
        Self::Count(0)
    }
}

/// HTML wrapper element used by [`TypeAnimation`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Wrapper {
    P,
    Div,
    #[default]
    Span,
    Strong,
    A,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    B,
}

/// Props for [`TypeAnimation`].
///
/// The component intentionally compares props as always equal, matching the
/// React implementation's permanent memoization/immutability behavior. If you
/// need changed props to take effect, mount a new component instance with a
/// different key.
#[derive(Clone, Props)]
pub struct TypeAnimationProps {
    /// Animation sequence: text, delays in milliseconds, and callbacks.
    pub sequence: Vec<SequenceElement>,

    /// Finite or infinite repeat behavior. Default: no repeats.
    #[props(default)]
    pub repeat: Repeat,

    /// Wrapper element. Default: [`Wrapper::Span`].
    #[props(default)]
    pub wrapper: Wrapper,

    /// Show the default blinking cursor. Default: `true`.
    #[props(default = true)]
    pub cursor: bool,

    /// Typing speed. Default: `Speed::Preset(40)`.
    #[props(default)]
    pub speed: Speed,

    /// Deletion speed. Default: same as `speed`.
    #[props(default)]
    pub deletion_speed: Option<Speed>,

    /// If true, deletions are instant and only writing is animated.
    #[props(default)]
    pub omit_deletion_animation: bool,

    /// If true, initially render the first string in `sequence` without typing
    /// it. Default matches the React source: `false`.
    #[props(default)]
    pub pre_render_first_string: bool,

    /// Optional custom splitter. Default: `text.chars()`.
    #[props(default)]
    pub splitter: Option<StringSplitter>,

    /// Class applied to the wrapper.
    #[props(default)]
    pub class: Option<String>,

    /// Inline style string applied to the wrapper.
    #[props(default)]
    pub style: Option<String>,

    /// `aria-label` applied to the wrapper. When set, the animated visual text
    /// is rendered in an inner `aria-hidden="true"` span.
    #[props(default)]
    pub aria_label: Option<String>,

    /// `aria-hidden` applied to the wrapper.
    #[props(default)]
    pub aria_hidden: Option<String>,

    /// ARIA role applied to the wrapper.
    #[props(default)]
    pub role: Option<String>,
}

impl PartialEq for TypeAnimationProps {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

/// Dioxus typewriter animation component inspired by `react-type-animation`.
///
/// # Basic usage
///
/// ```rust,no_run
/// use dioxus::prelude::*;
/// use dioxus_type_animation::{Repeat, SequenceElement, Speed, TypeAnimation, Wrapper};
///
/// fn App() -> Element {
///     rsx! {
///         TypeAnimation {
///             sequence: vec![
///                 SequenceElement::from("We produce food for Mice"),
///                 SequenceElement::from(1000_u64),
///                 SequenceElement::from("We produce food for Hamsters"),
///                 SequenceElement::from(1000_u64),
///                 SequenceElement::from("We produce food for Guinea Pigs"),
///                 SequenceElement::from(1000_u64),
///                 SequenceElement::from("We produce food for Chinchillas"),
///                 SequenceElement::from(1000_u64),
///             ],
///             wrapper: Wrapper::Span,
///             speed: Speed::Preset(50),
///             style: Some("font-size: 2em; display: inline-block;".to_string()),
///             repeat: Repeat::Infinite,
///         }
///     }
/// }
/// ```
///
/// # Sequence items
///
/// The `sequence` prop accepts text transitions, delays in milliseconds, and
/// callbacks.
///
/// ```rust,no_run
/// use dioxus::prelude::*;
/// use dioxus_type_animation::{SequenceElement, TypeAnimation};
///
/// fn App() -> Element {
///     rsx! {
///         TypeAnimation {
///             sequence: vec![
///                 SequenceElement::from("Typing..."),
///                 SequenceElement::from(750_u64),
///                 SequenceElement::from(|| println!("Done typing")),
///                 SequenceElement::from("Finished."),
///             ],
///         }
///     }
/// }
/// ```
///
/// # Repeat behavior
///
/// `Repeat::Count(n)` runs the animation once plus `n` repeats. Use
/// `Repeat::Infinite` to loop forever.
///
/// ```rust,no_run
/// use dioxus::prelude::*;
/// use dioxus_type_animation::{Repeat, SequenceElement, TypeAnimation};
///
/// fn App() -> Element {
///     rsx! {
///         TypeAnimation {
///             sequence: vec![
///                 SequenceElement::from("Loop me"),
///                 SequenceElement::from(1000_u64),
///             ],
///             repeat: Repeat::Count(3),
///         }
///     }
/// }
/// ```
///
/// # Speed options
///
/// `Speed::Preset(value)` mirrors the React library's numeric `speed` prop and
/// normalizes to `abs(value - 100)` milliseconds per keystroke. Use
/// `Speed::KeyStrokeDelayInMs(value)` for a direct base delay in milliseconds.
///
/// ```rust,no_run
/// use dioxus::prelude::*;
/// use dioxus_type_animation::{SequenceElement, Speed, TypeAnimation};
///
/// fn App() -> Element {
///     rsx! {
///         TypeAnimation {
///             sequence: vec![
///                 SequenceElement::from("Type quickly"),
///                 SequenceElement::from(500_u64),
///                 SequenceElement::from("Delete slowly"),
///             ],
///             speed: Speed::Preset(80),
///             deletion_speed: Some(Speed::KeyStrokeDelayInMs(120)),
///         }
///     }
/// }
/// ```
///
/// # Omit deletion animation
///
/// Set `omit_deletion_animation` to skip animated deletion steps and only
/// animate newly written text.
///
/// ```rust,no_run
/// use dioxus::prelude::*;
/// use dioxus_type_animation::{SequenceElement, TypeAnimation};
///
/// fn App() -> Element {
///     rsx! {
///         TypeAnimation {
///             sequence: vec![
///                 SequenceElement::from("Dioxus is fun"),
///                 SequenceElement::from(1000_u64),
///                 SequenceElement::from("Dioxus is fast"),
///             ],
///             omit_deletion_animation: true,
///         }
///     }
/// }
/// ```
///
/// # Wrapper elements
///
/// The default wrapper is [`Wrapper::Span`]. You can choose from `p`, `div`,
/// `span`, `strong`, `a`, `h1`-`h6`, and `b` via [`Wrapper`].
///
/// ```rust,no_run
/// use dioxus::prelude::*;
/// use dioxus_type_animation::{SequenceElement, TypeAnimation, Wrapper};
///
/// fn App() -> Element {
///     rsx! {
///         TypeAnimation {
///             wrapper: Wrapper::H1,
///             sequence: vec![SequenceElement::from("Animated heading")],
///         }
///     }
/// }
/// ```
///
/// # Cursor styling
///
/// The blinking cursor is enabled by default. Disable it with `cursor: false`.
///
/// ```rust,no_run
/// use dioxus::prelude::*;
/// use dioxus_type_animation::{SequenceElement, TypeAnimation};
///
/// fn App() -> Element {
///     rsx! {
///         TypeAnimation {
///             sequence: vec![SequenceElement::from("No cursor")],
///             cursor: false,
///         }
///     }
/// }
/// ```
///
/// # Pre-render the first string
///
/// Set `pre_render_first_string` to render the first string immediately before
/// animation starts.
///
/// ```rust,no_run
/// use dioxus::prelude::*;
/// use dioxus_type_animation::{SequenceElement, TypeAnimation};
///
/// fn App() -> Element {
///     rsx! {
///         TypeAnimation {
///             sequence: vec![
///                 SequenceElement::from("Already visible"),
///                 SequenceElement::from(1000_u64),
///                 SequenceElement::from("Then animated"),
///             ],
///             pre_render_first_string: true,
///         }
///     }
/// }
/// ```
///
/// # Accessibility attributes
///
/// You can pass `aria_label`, `aria_hidden`, and `role`. When `aria_label` is
/// set, the animated visual text is rendered inside an inner
/// `aria-hidden="true"` span.
///
/// ```rust,no_run
/// use dioxus::prelude::*;
/// use dioxus_type_animation::{SequenceElement, TypeAnimation};
///
/// fn App() -> Element {
///     rsx! {
///         TypeAnimation {
///             sequence: vec![SequenceElement::from("Fast-changing animated text")],
///             aria_label: Some("Animated product tagline".to_string()),
///             role: Some("text".to_string()),
///         }
///     }
/// }
/// ```
///
/// # Custom splitter
///
/// The default splitter uses `text.chars()`. Provide a custom [`StringSplitter`]
/// for grapheme-aware animation or other splitting behavior.
///
/// ```rust,no_run
/// use dioxus::prelude::*;
/// use dioxus_type_animation::{SequenceElement, StringSplitter, TypeAnimation};
/// use std::rc::Rc;
///
/// fn App() -> Element {
///     let splitter: StringSplitter = Rc::new(|text: &str| {
///         text.chars().map(|char| char.to_string()).collect()
///     });
///
///     rsx! {
///         TypeAnimation {
///             sequence: vec![SequenceElement::from("👨‍👩‍👧‍👦 family")],
///             splitter: Some(splitter),
///         }
///     }
/// }
/// ```
pub fn TypeAnimation(props: TypeAnimationProps) -> Element {
    let initial_text = if props.pre_render_first_string {
        first_string(&props.sequence).unwrap_or_default()
    } else {
        String::new()
    };

    let mut displayed = use_signal(|| initial_text.clone());

    {
        let sequence = props.sequence.clone();
        let repeat = props.repeat;
        let speed = normalize_speed(props.speed);
        let deletion_speed = normalize_speed(props.deletion_speed.unwrap_or(props.speed));
        let omit_deletion_animation = props.omit_deletion_animation;
        let splitter = props.splitter.clone().unwrap_or_else(default_splitter);
        let starting_text = initial_text;

        use_future(move || {
            let sequence = sequence.clone();
            let splitter = splitter.clone();
            let starting_text = starting_text.clone();
            async move {
                let config = AnimationConfig {
                    repeat,
                    speed,
                    deletion_speed,
                    omit_deletion_animation,
                };

                run_animation(&sequence, &splitter, config, starting_text, &mut displayed).await;
            }
        });
    }

    let text = displayed.read().clone();
    let class = final_class_name(props.cursor, props.class.as_deref());
    let style = props.style.unwrap_or_default();
    let aria_label = props.aria_label.unwrap_or_default();
    let aria_hidden = props.aria_hidden.unwrap_or_default();
    let role = props.role.unwrap_or_default();
    let use_inner_accessibility_span = !aria_label.is_empty();
    let render_data = RenderData {
        text,
        class,
        style,
        aria_label,
        aria_hidden,
        role,
        use_inner_accessibility_span,
    };

    match props.wrapper {
        Wrapper::P => render_p(render_data),
        Wrapper::Div => render_div(render_data),
        Wrapper::Span => render_span(render_data),
        Wrapper::Strong => render_strong(render_data),
        Wrapper::A => render_a(render_data),
        Wrapper::H1 => render_h1(render_data),
        Wrapper::H2 => render_h2(render_data),
        Wrapper::H3 => render_h3(render_data),
        Wrapper::H4 => render_h4(render_data),
        Wrapper::H5 => render_h5(render_data),
        Wrapper::H6 => render_h6(render_data),
        Wrapper::B => render_b(render_data),
    }
}

struct RenderData {
    text: String,
    class: String,
    style: String,
    aria_label: String,
    aria_hidden: String,
    role: String,
    use_inner_accessibility_span: bool,
}

fn render_p(data: RenderData) -> Element {
    let RenderData {
        text,
        class,
        style,
        aria_label,
        aria_hidden,
        role,
        use_inner_accessibility_span,
    } = data;
    rsx! {
        style { {CURSOR_CSS} }
        p { class, style, role, "aria-label": aria_label, "aria-hidden": aria_hidden,
            if use_inner_accessibility_span { span { "aria-hidden": "true", "{text}" } } else { "{text}" }
        }
    }
}

fn render_div(data: RenderData) -> Element {
    let RenderData {
        text,
        class,
        style,
        aria_label,
        aria_hidden,
        role,
        use_inner_accessibility_span,
    } = data;
    rsx! {
        style { {CURSOR_CSS} }
        div { class, style, role, "aria-label": aria_label, "aria-hidden": aria_hidden,
            if use_inner_accessibility_span { span { "aria-hidden": "true", "{text}" } } else { "{text}" }
        }
    }
}

fn render_span(data: RenderData) -> Element {
    let RenderData {
        text,
        class,
        style,
        aria_label,
        aria_hidden,
        role,
        use_inner_accessibility_span,
    } = data;
    rsx! {
        style { {CURSOR_CSS} }
        span { class, style, role, "aria-label": aria_label, "aria-hidden": aria_hidden,
            if use_inner_accessibility_span { span { "aria-hidden": "true", "{text}" } } else { "{text}" }
        }
    }
}

fn render_strong(data: RenderData) -> Element {
    let RenderData {
        text,
        class,
        style,
        aria_label,
        aria_hidden,
        role,
        use_inner_accessibility_span,
    } = data;
    rsx! {
        style { {CURSOR_CSS} }
        strong { class, style, role, "aria-label": aria_label, "aria-hidden": aria_hidden,
            if use_inner_accessibility_span { span { "aria-hidden": "true", "{text}" } } else { "{text}" }
        }
    }
}

fn render_a(data: RenderData) -> Element {
    let RenderData {
        text,
        class,
        style,
        aria_label,
        aria_hidden,
        role,
        use_inner_accessibility_span,
    } = data;
    rsx! {
        style { {CURSOR_CSS} }
        a { class, style, role, "aria-label": aria_label, "aria-hidden": aria_hidden,
            if use_inner_accessibility_span { span { "aria-hidden": "true", "{text}" } } else { "{text}" }
        }
    }
}

fn render_h1(data: RenderData) -> Element {
    let RenderData {
        text,
        class,
        style,
        aria_label,
        aria_hidden,
        role,
        use_inner_accessibility_span,
    } = data;
    rsx! {
        style { {CURSOR_CSS} }
        h1 { class, style, role, "aria-label": aria_label, "aria-hidden": aria_hidden,
            if use_inner_accessibility_span { span { "aria-hidden": "true", "{text}" } } else { "{text}" }
        }
    }
}

fn render_h2(data: RenderData) -> Element {
    let RenderData {
        text,
        class,
        style,
        aria_label,
        aria_hidden,
        role,
        use_inner_accessibility_span,
    } = data;
    rsx! {
        style { {CURSOR_CSS} }
        h2 { class, style, role, "aria-label": aria_label, "aria-hidden": aria_hidden,
            if use_inner_accessibility_span { span { "aria-hidden": "true", "{text}" } } else { "{text}" }
        }
    }
}

fn render_h3(data: RenderData) -> Element {
    let RenderData {
        text,
        class,
        style,
        aria_label,
        aria_hidden,
        role,
        use_inner_accessibility_span,
    } = data;
    rsx! {
        style { {CURSOR_CSS} }
        h3 { class, style, role, "aria-label": aria_label, "aria-hidden": aria_hidden,
            if use_inner_accessibility_span { span { "aria-hidden": "true", "{text}" } } else { "{text}" }
        }
    }
}

fn render_h4(data: RenderData) -> Element {
    let RenderData {
        text,
        class,
        style,
        aria_label,
        aria_hidden,
        role,
        use_inner_accessibility_span,
    } = data;
    rsx! {
        style { {CURSOR_CSS} }
        h4 { class, style, role, "aria-label": aria_label, "aria-hidden": aria_hidden,
            if use_inner_accessibility_span { span { "aria-hidden": "true", "{text}" } } else { "{text}" }
        }
    }
}

fn render_h5(data: RenderData) -> Element {
    let RenderData {
        text,
        class,
        style,
        aria_label,
        aria_hidden,
        role,
        use_inner_accessibility_span,
    } = data;
    rsx! {
        style { {CURSOR_CSS} }
        h5 { class, style, role, "aria-label": aria_label, "aria-hidden": aria_hidden,
            if use_inner_accessibility_span { span { "aria-hidden": "true", "{text}" } } else { "{text}" }
        }
    }
}

fn render_h6(data: RenderData) -> Element {
    let RenderData {
        text,
        class,
        style,
        aria_label,
        aria_hidden,
        role,
        use_inner_accessibility_span,
    } = data;
    rsx! {
        style { {CURSOR_CSS} }
        h6 { class, style, role, "aria-label": aria_label, "aria-hidden": aria_hidden,
            if use_inner_accessibility_span { span { "aria-hidden": "true", "{text}" } } else { "{text}" }
        }
    }
}

fn render_b(data: RenderData) -> Element {
    let RenderData {
        text,
        class,
        style,
        aria_label,
        aria_hidden,
        role,
        use_inner_accessibility_span,
    } = data;
    rsx! {
        style { {CURSOR_CSS} }
        b { class, style, role, "aria-label": aria_label, "aria-hidden": aria_hidden,
            if use_inner_accessibility_span { span { "aria-hidden": "true", "{text}" } } else { "{text}" }
        }
    }
}

#[derive(Clone, Copy)]
struct AnimationConfig {
    repeat: Repeat,
    speed: u64,
    deletion_speed: u64,
    omit_deletion_animation: bool,
}

async fn run_animation(
    sequence: &[SequenceElement],
    splitter: &StringSplitter,
    config: AnimationConfig,
    mut current_text: String,
    displayed: &mut Signal<String>,
) {
    match config.repeat {
        Repeat::Count(count) => {
            for _ in 0..=count {
                current_text = run_sequence_once(
                    sequence,
                    splitter,
                    config.speed,
                    config.deletion_speed,
                    config.omit_deletion_animation,
                    current_text,
                    displayed,
                )
                .await;
            }
        }
        Repeat::Infinite => loop {
            current_text = run_sequence_once(
                sequence,
                splitter,
                config.speed,
                config.deletion_speed,
                config.omit_deletion_animation,
                current_text,
                displayed,
            )
            .await;
        },
    }
}

async fn run_sequence_once(
    sequence: &[SequenceElement],
    splitter: &StringSplitter,
    speed: u64,
    deletion_speed: u64,
    omit_deletion_animation: bool,
    mut current_text: String,
    displayed: &mut Signal<String>,
) -> String {
    for item in sequence {
        match item {
            SequenceElement::Text(next_text) => {
                let edits = edits_for_transition(&current_text, next_text, splitter);
                perform_edits(
                    displayed,
                    &edits,
                    speed,
                    deletion_speed,
                    omit_deletion_animation,
                    &mut current_text,
                )
                .await;
                current_text = next_text.clone();
            }
            SequenceElement::Delay(ms) => wait(*ms).await,
            SequenceElement::Callback(callback) => callback(),
        }
    }

    current_text
}

async fn perform_edits(
    displayed: &mut Signal<String>,
    edits: &[String],
    speed: u64,
    deletion_speed: u64,
    omit_deletion_animation: bool,
    current_text: &mut String,
) {
    let filtered_edits = if omit_deletion_animation {
        omit_deletion_edits(edits)
    } else {
        edits.to_vec()
    };

    for snippet in filtered_edits {
        let op = if snippet.is_empty() || current_text.chars().count() > snippet.chars().count() {
            Operation::Deletion
        } else {
            Operation::Writing
        };

        *current_text = snippet.clone();
        displayed.set(snippet);

        let base_delay = match op {
            Operation::Writing => speed,
            Operation::Deletion => deletion_speed,
        };
        wait(jittered_delay(base_delay)).await;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operation {
    Writing,
    Deletion,
}

async fn wait(ms: u64) {
    TimeoutFuture::new(ms.min(u32::MAX as u64) as u32).await;
}

fn default_splitter() -> StringSplitter {
    Rc::new(|text: &str| text.chars().map(|c| c.to_string()).collect())
}

fn first_string(sequence: &[SequenceElement]) -> Option<String> {
    sequence.iter().find_map(|item| match item {
        SequenceElement::Text(text) => Some(text.clone()),
        _ => None,
    })
}

fn normalize_speed(speed: Speed) -> u64 {
    match speed {
        Speed::Preset(value) => value.abs_diff(100),
        Speed::KeyStrokeDelayInMs(value) => value,
    }
}

fn jittered_delay(base: u64) -> u64 {
    if base == 0 {
        return 0;
    }

    let jitter = fastrand::f64() - 0.5;
    ((base as f64) + (base as f64 * jitter)).max(0.0).round() as u64
}

fn final_class_name(cursor: bool, class: Option<&str>) -> String {
    match (cursor, class.filter(|value| !value.is_empty())) {
        (true, Some(class)) => format!("{CURSOR_CLASS} {class}"),
        (true, None) => CURSOR_CLASS.to_string(),
        (false, Some(class)) => class.to_string(),
        (false, None) => String::new(),
    }
}

fn edits_for_transition(start: &str, end: &str, splitter: &StringSplitter) -> Vec<String> {
    let overlap = get_overlap(start, end);
    deleter(start, splitter, overlap)
        .into_iter()
        .chain(writer(end, splitter, overlap))
        .collect()
}

fn writer(text: &str, splitter: &StringSplitter, mut start_index: usize) -> Vec<String> {
    let split_text = splitter(text);
    let end_index = split_text.len();
    let mut snippets = Vec::new();

    while start_index < end_index {
        start_index += 1;
        snippets.push(split_text[..start_index].join(""));
    }

    snippets
}

fn deleter(text: &str, splitter: &StringSplitter, start_index: usize) -> Vec<String> {
    let split_text = splitter(text);
    let mut end_index = split_text.len();
    let mut snippets = Vec::new();

    while end_index > start_index {
        end_index -= 1;
        snippets.push(split_text[..end_index].join(""));
    }

    snippets
}

fn get_overlap(start: &str, end: &str) -> usize {
    let mut index = 0;

    for (start_char, end_char) in start.chars().zip(end.chars()) {
        if start_char != end_char {
            return index;
        }
        index += 1;
    }

    index.min(start.chars().count()).min(end.chars().count())
}

fn omit_deletion_edits(edits: &[String]) -> Vec<String> {
    let mut slice_point = 0;

    for i in 1..edits.len() {
        let prev_len = edits[i - 1].chars().count();
        let curr_len = edits[i].chars().count();

        if curr_len > prev_len || edits[i].is_empty() {
            slice_point = i;
            break;
        }
    }

    edits[slice_point..].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn speed_presets_match_react_normalization() {
        assert_eq!(normalize_speed(Speed::Preset(40)), 60);
        assert_eq!(normalize_speed(Speed::Preset(99)), 1);
        assert_eq!(normalize_speed(Speed::Preset(1)), 99);
        assert_eq!(normalize_speed(Speed::KeyStrokeDelayInMs(125)), 125);
    }

    #[test]
    fn overlap_finds_common_prefix() {
        assert_eq!(
            get_overlap("We produce food for Mice", "We produce food for Hamsters"),
            20
        );
        assert_eq!(get_overlap("abc", "abcde"), 3);
        assert_eq!(get_overlap("abc", "xyz"), 0);
    }

    #[test]
    fn transition_deletes_to_overlap_then_writes() {
        let splitter = default_splitter();
        let edits = edits_for_transition("abc", "abd", &splitter);
        assert_eq!(edits, vec!["ab".to_string(), "abd".to_string()]);
    }

    #[test]
    fn omit_deletion_starts_at_deletion_end_state() {
        let edits = vec![
            "abc".to_string(),
            "ab".to_string(),
            "a".to_string(),
            "ax".to_string(),
        ];

        assert_eq!(omit_deletion_edits(&edits), vec!["ax".to_string()]);
    }

    #[test]
    fn class_name_matches_cursor_and_custom_class_settings() {
        assert_eq!(final_class_name(true, None), CURSOR_CLASS);
        assert_eq!(
            final_class_name(true, Some("hero")),
            format!("{CURSOR_CLASS} hero")
        );
        assert_eq!(final_class_name(false, Some("hero")), "hero");
        assert_eq!(final_class_name(false, None), "");
    }
}

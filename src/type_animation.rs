use dioxus::prelude::*;

use crate::{
    CURSOR_CSS,
    animation::{
        AnimationConfig, default_splitter, final_class_name, first_string, normalize_speed,
        run_animation,
    },
    repeat::Repeat,
    sequence::{SequenceElement, StringSplitter},
    speed::Speed,
    wrapper::Wrapper,
};

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
/// Props for [`TypeAnimation`](crate::TypeAnimation).
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

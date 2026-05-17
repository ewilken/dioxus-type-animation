use std::rc::Rc;

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

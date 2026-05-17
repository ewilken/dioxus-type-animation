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

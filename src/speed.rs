const DEFAULT_SPEED: u64 = 40;

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

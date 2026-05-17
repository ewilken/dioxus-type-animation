/// HTML wrapper element used by [`TypeAnimation`](crate::TypeAnimation).
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

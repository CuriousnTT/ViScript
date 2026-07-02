#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span  {
    pub fn new(start: usize, end: usize) -> Self{
        assert!(start <= end);
        Self { start, end }
    }

    /// Returns a Span spanning from start of self to end of other.
    pub fn until(self, other: Span) -> Span {
        Span {
            start: self.start,
            end: other.end,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub value: T,
    pub span: Span,
}

impl<T: std::fmt::Debug> std::fmt::Display for Spanned<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

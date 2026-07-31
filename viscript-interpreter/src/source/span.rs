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

    /// Shifts both start and end by 'offset'
    pub fn offset(self, offset: usize) -> Span {
        Span {
            start: self.start + offset,
            end: self.end + offset
        }
    }

    /// Offset convencience for when you have a base position
    pub fn offset_by(self, base:usize) -> Span {
        self.offset(base)
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

impl<T> Spanned<T> {
    /// Apply offset to the contained span
    pub fn offset(self, offset: usize) -> Spanned<T> {
        Spanned {
            value: self.value,
            span: self.span.offset(offset),
        }
    }

    /// mutating offset
    pub fn offset_in_place(&mut self, offset: usize) {
        self.span = self.span.offset(offset)
    }
}
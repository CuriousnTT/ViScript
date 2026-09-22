/// Describes different kinds of tuples
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TupleLabelStyle {
    /// Valid
    Unlabeled,
    /// Valid
    Labeled,
    // Valid in type declaration if it resolves correctly
    Unclear,
    // Invalid
    Mixed,
    /// Special case of overlap between type system and value system
    Empty,
}

impl TupleLabelStyle {
    /// returns true for TupleLabelStyles which
    /// an unclear case can resolve to
    pub fn is_resolution_target(&self) -> bool {
        matches!(
            self,
            Self::Labeled
            | Self::Unlabeled
        )
    }
}
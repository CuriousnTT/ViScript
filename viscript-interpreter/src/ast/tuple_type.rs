use crate::ast::{
    tuple_label::TupleLabelStyle,
    tuple_type_position::TupleTypePosition,
};
use crate::source::span::Spanned;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TupleType {
    pub positions: Vec<Spanned<TupleTypePosition>>,
}

impl TupleType {
    pub fn is_labeled(&self) -> bool {
        !self.is_empty() &&
        self.positions.iter().all(
            |p| p.value.is_labeled()
        )
    }

    pub fn is_unlabeled(&self) -> bool {
        !self.is_empty() &&
        self.positions.iter().all(
            |p| p.value.is_unlabeled()
        )
    }

    pub fn is_resolved(&self) -> bool {
        !self.is_empty() &&
        self.positions.iter().all(
            |p| p.value.is_resolved()
        )
    }

    /// Mixed is not valid
    pub fn is_mixed(&self) -> bool {
        !self.is_empty() &&
        self.positions.iter().any(
            |p| p.value.is_labeled()
        ) && self.positions.iter().any(
            |p| p.value.is_unlabeled()
        )
    }

    pub fn is_empty(&self) -> bool {
        self.positions.is_empty()
    }

    pub fn label_style(&self) -> TupleLabelStyle {
        if self.is_empty() {
            return TupleLabelStyle::Empty
        }

        let has_labeled = self.positions.iter()
            .any(|p| p.value.is_labeled());

        let has_unlabeled = self.positions.iter()
            .any(|p| p.value.is_unlabeled());

        if has_labeled && has_unlabeled {
            return TupleLabelStyle::Mixed
        }

        if !self.is_resolved() {
            return TupleLabelStyle::Unclear
        };

        if has_labeled {
            return TupleLabelStyle::Labeled
        } else {
            return TupleLabelStyle::Unlabeled
        }
    }

    pub fn resolve_as(self, style: TupleLabelStyle) -> Self {
        assert!(!self.is_resolved());
        assert!(style.is_resolution_target());

        match style {
            TupleLabelStyle::Labeled => Self {
                positions: self.positions.into_iter().map(
                    |p| p.map(TupleTypePosition::resolve_as_labeled)
                ).collect(),
            },

            TupleLabelStyle::Unlabeled => Self {
                positions: self.positions.into_iter().map(
                    |p| p.map(TupleTypePosition::resolve_as_unlabeled)
                ).collect(),
            },

            TupleLabelStyle::Mixed
            | TupleLabelStyle::Empty
            | TupleLabelStyle::Unclear => unreachable!("not a resolvable label style")
        }
    }
}
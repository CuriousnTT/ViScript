use crate::ast::{
    tuple_label::TupleLabelStyle,
    tuple_position::TuplePosition,
};
use crate::source::span::Spanned;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tuple {
    pub positions: Vec<Spanned<TuplePosition>>,
}

impl Tuple {
    pub fn label_style(&self) -> TupleLabelStyle {
        let mut labeled = false;
        let mut unlabeled = false;

        if self.positions.is_empty() {
            return TupleLabelStyle::Empty
        }

        for pos in &self.positions {
            if pos.value.is_labeled() {
                labeled = true;
            } else {
                unlabeled = true;
            }

            match (labeled, unlabeled) {
                (true, true) => return TupleLabelStyle::Mixed,
                _ => {}
            }
        }
        
        match (labeled, unlabeled) {
            (true, false) => TupleLabelStyle::Labeled,
            (false, true) => TupleLabelStyle::Unlabeled,
            (true, true) | (false, false) => unreachable!()
        }
    }
}
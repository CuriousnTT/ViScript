use crate::ast::tuple_position::TuplePosition;
use crate::source::span::Spanned;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tuple {
    pub positions: Vec<Spanned<TuplePosition>>,
}
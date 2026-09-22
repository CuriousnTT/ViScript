use crate::ast::{
    expression::Expression,
    statement::Statement,
};
use crate::source::span::Spanned;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TuplePosition {
    pub statements: Vec<Spanned<Statement>>,
    pub value: Spanned<Expression>,
    pub label: Option<Spanned<String>>,
}

impl TuplePosition {
    pub fn is_labeled(&self) -> bool {
        self.label.is_some()
    }
}
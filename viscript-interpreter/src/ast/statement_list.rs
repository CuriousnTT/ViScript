use crate::ast::statement::Statement;
use crate::source::span::{
    Span,
    Spanned,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatementList {
    pub statements: Vec<Spanned<Statement>>,
    pub span: Span,
}
pub type Program = StatementList;
pub type Block = StatementList;
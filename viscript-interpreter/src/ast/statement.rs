use crate::ast::{
    expression::Expression,
    statement_list::Block,
};
use crate::source::span::Spanned;
use crate::token::keyword::Keyword;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    VariableDeclaration {
        keyword: Spanned<Keyword>,
        identifier: Spanned<String>,
        value: Spanned<Expression>,
    },
    FunctionDeclaration {
        keyword: Spanned<Keyword>,
        identifier: Spanned<String>,
        body: Block,
    },
    Expression(Spanned<Expression>),
}
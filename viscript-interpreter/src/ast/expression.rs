use crate::ast::{
    tuple::Tuple
};
use crate::source::span::Spanned;
use crate::token::operator::Operator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    // primary
    NumericLiteral(Spanned<String>),
    Identifier(Spanned<String>),
    Tuple(Spanned<Tuple>),

    // postfixes
    MemberAccess {
        object: Box<Spanned<Expression>>,
        member: Spanned<String>,
    },
    Call {
        callable: Box<Spanned<Expression>>,
        arguments: Spanned<Tuple>,
    },

    // Sequences
    Binary {
        left: Box<Spanned<Expression>>,
        operator: Spanned<Operator>,
        right: Box<Spanned<Expression>>,
    },
}
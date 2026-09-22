use crate::ast::{
    tuple::Tuple,
    template::Template,
};
use crate::source::span::Spanned;
use crate::token::operator::Operator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    // primary
    Identifier(String),
    /// Persists as Null until semantic analysis
    NullLiteral,
    NumericLiteral(String),
    StringLiteral(String),
    Tuple(Spanned<Tuple>),
    Template(Spanned<Template>),
    /// Consumes NullLiteral in semantic analysis
    UndefinedLiteral,
    

    // postfixes
    MemberAccess {
        object: Box<Spanned<Expression>>,
        member: Spanned<String>,
    },
    Call {
        callable: Box<Spanned<Expression>>,
        arguments: Spanned<Tuple>,
    },

    // secondary
    Binary {
        left: Box<Spanned<Expression>>,
        operator: Spanned<Operator>,
        right: Box<Spanned<Expression>>,
    },
    Assignment {
        target: Box<Spanned<Expression>>,
        operator: Spanned<Operator>,
        value: Box<Spanned<Expression>>,
    }
}
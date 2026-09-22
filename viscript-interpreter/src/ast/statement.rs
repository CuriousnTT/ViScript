use crate::ast::{
    directive::Directive,
    expression::Expression,
    statement_list::Block,
    tuple::Tuple,
    type_expression::TypeExpression,
};
use crate::source::span::Spanned;
use crate::token::keyword::Keyword;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Directed {
        statement: Box<Spanned<Statement>>,
        directives: Vec<Spanned<Directive>>,
    },
    ExportDeclaration {
        keyword: Spanned<Keyword>,
        default: Option<Spanned<Keyword>>,
        declaration: Box<Spanned<Statement>>,
    },
    Expression(Spanned<Expression>),
    FunctionDeclaration {
        async_: Option<Spanned<Keyword>>,
        keyword: Spanned<Keyword>,
        identifier: Spanned<String>,
        parameters: Spanned<TypeExpression>,
        body: Block,
    },
    If {
        condition: Spanned<Tuple>,
        then: Block,
        else_: Option<Box<Spanned<Statement>>>,
    },
    Return {
        keyword: Spanned<Keyword>,
        value: Option<Spanned<Expression>>,
    },
    VariableDeclaration {
        keyword: Spanned<Keyword>,
        identifier: Spanned<String>,
        value: Option<Spanned<Expression>>,
    },
}
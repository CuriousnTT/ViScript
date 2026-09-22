use crate::ast::expression::Expression;
use crate::source::span::Spanned;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplatePart {
    Text(Spanned<String>),
    Expression(Spanned<Expression>),
}
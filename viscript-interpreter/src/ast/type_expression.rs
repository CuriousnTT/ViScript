use crate::ast::{
    intrinsic_type::IntrinsicType,
    tuple_type::TupleType,
};
use crate::source::span::Spanned;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeExpression {
    Identifier(String),
    Tuple(Spanned<TupleType>),
    Intrinsic(IntrinsicType)
}
use crate::lexer::{
    directive::Directive,
    keyword::Keyword,
    literal::Literal,
    operator::Operator,
    punctuation::Punctuation,
};

/// Main Token abstraction
#[derive(Debug, Clone)]
pub enum Token {
    Keyword(Keyword),
    Identifier(String),

    Literal(Literal),

    // Syntax
    Operator(Operator),
    Punctuation(Punctuation),
    Directive(Directive),
    EOF,
}

/// 1-dimensional look at recognized token types.
#[derive(Debug, Clone)]
pub enum TokenKind {
    Keyword,
    Skip,
    Null,
    Boolean,
    Undefined,
    BigInt,
    Numeric,
    String,
    RegExp,
    Identifier,
    Operator,
    Punctuation,
    Directive,
}
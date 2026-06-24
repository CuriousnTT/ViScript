use crate::lexer::keyword::Keyword;
use crate::lexer::literal::Literal;

#[derive(Debug, Clone)]
pub enum Token {
    Keyword(Keyword),
    Identifier(String),

    Literal(Literal),

    // Syntax
    Operator(String),
    Punctuation(String),
    Directive(String),
    EOF,
}

#[derive(Debug, Clone)]
pub enum TokenKind {
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
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

impl Token {
        pub fn can_end_expression(&self) -> bool {
        match self {
            Token::Literal(_) | Token::Identifier(_) => true,

            // complete an expression
            Token::Keyword(
                Keyword::This
                | Keyword::Super
            ) => true,

            Token::Punctuation(
                Punctuation::RightParen
                | Punctuation::RightBracket
                | Punctuation::RightBrace
            ) => true,

            Token::Operator(
                Operator::Division
                | Operator::DivisionAssignment
                | Operator::Increment
                | Operator::Decrement
            ) => true,

            Token::Keyword(
                Keyword::Return
                | Keyword::Throw
                | Keyword::Break
                | Keyword::Continue
                | Keyword::Else
                | Keyword::Finally
            ) => true,

            _ => false,

        }
    }
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
use crate::lexer::{
    directive::Directive,
    keyword::Keyword,
    literal::Literal,
    operator::Operator,
    punctuation::Punctuation,
    template::Template,
};

/// Main Token abstraction
#[derive(Debug, Clone)]
pub enum Token {
    Keyword(Keyword),
    Identifier(String),

    Literal(Literal),
    Template(Template),

    // Syntax
    Operator(Operator),
    Punctuation(Punctuation),
    Directive(Directive),
    EOF,
}

impl Token {

    pub fn is_less_than(&self) -> bool {
        return matches!(
            self, Token::Operator(Operator::LessThan)
        )
    }
}

/// 1-dimensional look at regex-lexed Token kinds.
#[derive(Debug, Clone)]
pub enum TokenKind {
    Skip,
    Null,
    Boolean,
    Undefined,
    String,
    Identifier,
    Operator,
    Punctuation,
    Directive,
}
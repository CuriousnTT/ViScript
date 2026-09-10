use crate::token::{
    directive::Directive,
    keyword::Keyword,
    literal::Literal,
    operator::Operator,
    punctuation::Punctuation,
    template::Template,
    vsx::VSX,
};

/// Main Token abstraction
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Identifier(String),

    Literal(Literal),
    Template(Template),

    // VSX
    VSX(VSX),

    // Syntax
    Operator(Operator),
    Punctuation(Punctuation),
    Directive(Directive),
    EOF
}

impl Token {

    pub fn is_eof(&self) -> bool {
        matches!(self, Token::EOF)
    }

    pub fn is_less_than(&self) -> bool {
        return matches!(
            self, Token::Operator(Operator::LessThan)
        )
    }

    /// True for any operator whose spelling contains '<'
    pub fn is_less_than_family(&self) -> bool {
        matches!(
            self,
            Token::Operator(
                Operator::LessThan
                | Operator::LessThanOrEqual
                | Operator::LeftShift
                | Operator::LeftShiftAssignment
            )
        )
    }
}

/// 1-dimensional look at regex-lexed Token kinds.
#[derive(Debug, Clone)]
pub enum TokenKind {
    Keyword,
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
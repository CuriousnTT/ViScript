use std::fmt;

use crate::token::token::Token;
use crate::source::{
    span::{
        Span,
        Spanned,
    },
    sourcefile::SourceFile,
};

#[derive(Debug, Clone)]
pub enum ParserError {
    ExpectedExpression {
        found: Spanned<Token>
    },

    ExpectedStatement {
        found: Spanned<Token>,
    },

    UnexpectedToken {
        expected: Vec<Token>,
        found: Spanned<Token>,
    },

    UnexpectedEof {
        span: Span,
    },
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExpectedExpression { found, .. } => {
                write!(f, "Expected expression, found '{:?}'", found.value)
            },
            Self::ExpectedStatement { found } => {
                write!(f, "Expected token that could start statement, found '{:?}'", found.value)
            },
            Self::UnexpectedToken { expected, found } => {
                write!(f, "Expected '{:?}', found '{:?}'", expected, found.value)
            },
            Self::UnexpectedEof { .. } => {
                write!(f, "Unexpected end of file")
            },
        }
    }
}

impl std::error::Error for ParserError {}

impl ParserError {
    pub fn span(&self) -> Span {
        match self {
            Self::ExpectedExpression { found } => found.span,
            Self::ExpectedStatement { found } => found.span,
            Self::UnexpectedToken { found, .. } => found.span,
            Self::UnexpectedEof { span } => *span,
        }
    }

    pub fn report(&self, file: &SourceFile) -> String {
        let span = self.span();
        let (line, col) = file.line_column(span.start);// 0-based -> must be made 1-based for users

        let mut msg = format!(
            "error: {} at {}:{}\n",
            self,
            line + 1,
            col + 1,
        );
        msg.push_str(&file.get_error_context(span, 1));
        msg
    }
}
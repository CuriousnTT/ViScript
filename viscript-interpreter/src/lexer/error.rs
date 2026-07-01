use std::fmt;

use crate::utility::span::{SourceFile, Span};

#[derive(Debug)]
pub enum LexerError {
    UnexpectedCharacter {char: char, span: Span },
    UnknownDirective { name: String, span: Span },
    InvalidNumber { value: String, span: Span },
    InvalidBigInt { value: String, span: Span },
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedCharacter {char, span } => {
                write!(f, "Unexpected character: '{}'", char)
            }
            Self::UnknownDirective { name, span } => {
                write!(f, "Unknown directive '{}'", name)
            }
            Self::InvalidNumber { value, span } => {
                write!(f, "Invalid number '{}'", value)
            }
            Self::InvalidBigInt { value, span } => {
                write!(f, "Invalid BigInt '{}'", value)
            }
        }
    }
}

impl std::error::Error for LexerError {}

impl LexerError {
    pub fn span(&self) -> Span {
        match self {
            Self::UnexpectedCharacter { span, .. } => *span,
            Self::UnknownDirective { span, .. } => *span,
            Self::InvalidNumber { span, .. } => *span,
            Self::InvalidBigInt { span, .. } => *span,
        }
    }

    pub fn report(&self, file: &SourceFile) -> String {
        let span = self.span();
        let (line, col) = span.line_column(file);

        let mut msg = format!("error: {} at {}:{}\n", self, line, col);
        msg.push_str(&file.get_error_context(span, 1));
        msg
    }
}
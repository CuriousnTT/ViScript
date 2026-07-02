use std::fmt;

use crate::source::{
    span::Span,
    sourcefile::SourceFile,
};

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
        let (line, col) = file.line_column(span.start); // 0-based -> must be made 1-based for users

        let mut msg = format!(
            "error: {} at {}:{}\n",
            self,
            line + 1,
            col + 1
        );
        msg.push_str(&file.get_error_context(span, 1));
        msg
    }
}
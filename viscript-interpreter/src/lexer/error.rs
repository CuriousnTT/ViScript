use std::fmt;

use crate::source::{
    span::Span,
    sourcefile::SourceFile,
};

#[derive(Debug)]
pub enum LexerError {
    InvalidBigInt { value: String, span: Span },
    InvalidNumber { value: String, span: Span },
    InvalidRegExp { value: String, span: Span },
    UnexpectedCharacter {char: char, span: Span },
    UnknownDirective { name: String, span: Span },
    UnterminatedInterpolation { span: Span },
    UnterminatedRegex { text: String, span: Span },
    UnterminatedTemplate { span: Span },
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidNumber { value, .. } => {
                write!(f, "Invalid number '{}'", value)
            }
            Self::InvalidBigInt { value, .. } => {
                write!(f, "Invalid BigInt '{}'", value)
            }
            Self::InvalidRegExp { value, .. } => {
                write!(f, "Invalid RegExp '{}'", value)
            }
            Self::UnexpectedCharacter {char, .. } => {
                write!(f, "Unexpected character: '{}'", char)
            }
            Self::UnknownDirective { name, .. } => {
                write!(f, "Unknown directive '{}'", name)
            }
            Self::UnterminatedInterpolation { .. } => {
                write!(f, "Unterminated interpolation (missing '}}')")
            }
            Self::UnterminatedRegex { .. }=> {
                write!(f, "Unterminated regex (missing '/'")
            }
            Self::UnterminatedTemplate { .. } => {
                write!(f, "Unterminated template (missing '`'")
            }
        }
    }
}

impl std::error::Error for LexerError {}

impl LexerError {
    pub fn span(&self) -> Span {
        match self {
            Self::InvalidBigInt { span, .. } => *span,
            Self::InvalidNumber { span, .. } => *span,
            Self::InvalidRegExp { span, .. } => *span,
            Self::UnexpectedCharacter { span, .. } => *span,
            Self::UnknownDirective { span, .. } => *span,
            Self::UnterminatedInterpolation { span } => *span,
            Self::UnterminatedRegex { span , .. } => *span,
            Self::UnterminatedTemplate { span } => *span,
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
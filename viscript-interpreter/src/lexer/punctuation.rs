use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Punctuation {
    Dot,
    Colon,
    Semicolon,
    Comma,

    LeftParen,
    RightParen,

    LeftBrace,
    RightBrace,

    LeftBracket,
    RightBracket,
}

impl Punctuation {
    /// returns the textual representation of this punctuation token.
    #[inline]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Dot => ".",
            Self::Colon => ":",
            Self::Semicolon => ";",
            Self::Comma => ",",

            Self::LeftParen => "(",
            Self::RightParen => ")",

            Self::LeftBrace => "{",
            Self::RightBrace => "}",

            Self::LeftBracket => "[",
            Self::RightBracket => "]",
        }
    }

    #[inline]
    pub const fn from_char(s: char) -> Option<Self> {
        match s {
            '.' => Some(Self::Dot),
            ':' => Some(Self::Colon),
            ';' => Some(Self::Semicolon),
            ',' => Some(Self::Comma),

            '(' => Some(Self::LeftParen),
            ')' => Some(Self::RightParen),
            
            '{' => Some(Self::LeftBrace),
            '}' => Some(Self::RightBrace),
            
            '[' => Some(Self::LeftBracket),
            ']' => Some(Self::RightBracket),

            _ => None,
        }
    }
}

impl fmt::Display for Punctuation {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

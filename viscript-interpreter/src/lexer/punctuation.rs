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

    LeftSetBracket,
    RightSetBracket,
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

            Self::LeftSetBracket => "[|",
            Self::RightSetBracket => "|]",
        }
    }

    #[inline]
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "." => Some(Self::Dot),
            ":" => Some(Self::Colon),
            ";" => Some(Self::Semicolon),
            "," => Some(Self::Comma),

            "(" => Some(Self::LeftParen),
            ")" => Some(Self::RightParen),

            "{" => Some(Self::LeftBrace),
            "}" => Some(Self::RightBrace),

            "[" => Some(Self::LeftBracket),
            "]" => Some(Self::RightBracket),

            "[|" => Some(Self::LeftSetBracket),
            "|]" => Some(Self::RightSetBracket),

            _ => None,
        }
    }

    /// returns true for punctuation that is legal at the top level of a VSX
    /// opening tag's attribute list (outside of expressions)
    /// 
    /// NOTE: Rules apply differently in VSX tag *names*
    /// Ex: A VSX element can be defined as
    /// <NameSpace.Element
    ///   /*Attributes follow */
    /// />
    /// Dot *is* legal when used to access a namespace for a tag name.
    /// legal_in_vsx applies the post-name rules.
    pub fn legal_in_vsx(self) -> bool {
        matches!(
            self,
            // expression containers
            Self::LeftBrace
            | Self::RightBrace

            // Namespaced attribute names (xlink:href)
            | Self::Colon

            // not Self::Dot
        )
    }
}

impl fmt::Display for Punctuation {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

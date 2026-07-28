use regex::Regex;
use lazy_static::lazy_static;

use crate::lexer::token::TokenKind;


lazy_static! {
    ///Regex for skippable tokens
    pub static ref SKIP_REGEX: Vec<(Regex, TokenKind)> = vec![
        (Regex::new(r"[ \t\r\n]+").unwrap(), TokenKind::Skip),
        (Regex::new(r"//.*").unwrap(), TokenKind::Skip),
        (Regex::new(r"/\*[\s\S]*?\*/").unwrap(), TokenKind::Skip),
    ];

    /// Regex for all kinds of tokens
    pub static ref TOKEN_REGEX: Vec<(Regex, TokenKind)> = vec![
        // 1. Whitespace & comments (skip)
        (Regex::new(r"[ \t\r\n]+").unwrap(), TokenKind::Skip),
        (Regex::new(r"//.*").unwrap(), TokenKind::Skip),
        (Regex::new(r"/\*[\s\S]*?\*/").unwrap(), TokenKind::Skip),

        // 2. Reactive Directives
        (Regex::new(r"&[A-Za-z_$][A-Za-z0-9_$]*\b").unwrap(), TokenKind::Directive),


        (Regex::new(r"\bnull\b").unwrap(), TokenKind::Null),
        (Regex::new(r"\b(?:true|false)\b").unwrap(), TokenKind::Boolean),
        (Regex::new(r"\bundefined\b").unwrap(), TokenKind::Undefined),


        // Strings - single and double quotes
        (Regex::new(r#""(?:\\.|[^\\"])*""#).unwrap(), TokenKind::String),
        (Regex::new(r"'(?:\\.|[^\\'])*'").unwrap(), TokenKind::String),
    
        // 4.  Identifiers & keywords
        (Regex::new(r"[\p{L}_$][\p{L}\p{N}_$]*").unwrap(), TokenKind::Identifier),

        // Punctuation & operators (longest first!)
        (Regex::new(concat!(
            r"\+=|-=|\*=|/=|%=|\*\*=|",
            r"&&=|\|\|=|\?\?=|",
            r">>>=|>>>|<<=|>>=|<<|>>|",
            r"\^=|~|\?\?|\?\.|\.\.\.|",
            r"===|!==|==|!=|<=|>=|=>|",
            r"&&|\|\||\+\+|--|\*\*|",
            r"[+\-*/%=&|^!<>?]"
        )).unwrap(), TokenKind::Operator),
        (Regex::new(r"[.:,;(){}\[\]]").unwrap(), TokenKind::Punctuation),
    ];
}
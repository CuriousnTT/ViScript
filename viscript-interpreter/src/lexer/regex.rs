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

    // Regex used outside of token generation
    /// Matches all identifiers and keywords
    /// -> must be disentangled when that matters.
    pub static ref IDENTIFIER_REGEX: Regex =
        Regex::new(r"[\p{L}_$][\p{L}\p{N}_$]*").unwrap();

    /// like IDENTIFIER_REGEX but permits '-'
    pub static ref VSX_IDENTIFIER_REGEX: Regex =
        Regex::new(r"[\p{L}_$][\p{L}\p{N}_$-]*").unwrap();


    /// Regex for tokens that can occur inside a VSX tag
    pub static ref VSX_IN_TAG_REGEX: Vec<(Regex, TokenKind)> = vec![
        // 1. Skip
        (Regex::new(r"[ \t\r\n]+").unwrap(), TokenKind::Skip),
        (Regex::new(r"//.*").unwrap(), TokenKind::Skip),
        (Regex::new(r"/\*[\s\S]*?\*/").unwrap(), TokenKind::Skip),
        // 2. VSX Identifiers
        (VSX_IDENTIFIER_REGEX.clone(), TokenKind::Identifier),

        // 3. Strings - single and double quotes
        (Regex::new(r#""(?:\\.|[^\\"])*""#).unwrap(), TokenKind::String),
        (Regex::new(r"'(?:\\.|[^\\'])*'").unwrap(), TokenKind::String),

        // 4. Punctuation/operators that are valid outside of expressions
        (Regex::new(r"=").unwrap(), TokenKind::Operator), // assignment
        (Regex::new(r"[:{}.]").unwrap(), TokenKind::Punctuation),
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
        (IDENTIFIER_REGEX.clone(), TokenKind::Identifier),

        // Punctuation & operators (longest first!)
        (
            Regex::new(r"\[\||\|\]|[.:,;(){}\[\]]").unwrap(),
            TokenKind::Punctuation // Placed before Operators due to nature of overlap on |
        ),
        (Regex::new(concat!(
            r"\+=|-=|\*=|/=|%=|\*\*=|",
            r"&&=|\|\|=|\|=|\?\?=|",
            r">>>=|>>>|<<=|>>=|<<|>>|",
            r"\^=|~|\?\?|\?\.|\.\.\.|",
            r"===|!==|==|!=|<=|>=|=>|",
            r"&&|\|\||\+\+|--|\*\*|",
            r"[+\-*/%=&|^!<>?]"
        )).unwrap(), TokenKind::Operator),
    ];
}
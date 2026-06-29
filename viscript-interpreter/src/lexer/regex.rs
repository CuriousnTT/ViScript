use regex::Regex;
use lazy_static::lazy_static;

use crate::lexer::token::TokenKind;

lazy_static! {
    pub static ref TOKEN_REGEX: Vec<(Regex, TokenKind)> = vec![
        // 1. Whitespace & comments (skip)
        (Regex::new(r"[ \t\r\n]+").unwrap(), TokenKind::Skip),
        (Regex::new(r"//.*").unwrap(), TokenKind::Skip),
        (Regex::new(r"/\*[\s\S]*?\*/").unwrap(), TokenKind::Skip),

        // 2. Reactive Directives
        (Regex::new(r"&[A-Za-z_$][A-Za-z0-9_$]*\b").unwrap(), TokenKind::Directive),


        // 3. Literals - invalid first, then ordered by specificity
        //(Regex::new(r"\d+[eE][+-]?(_|\b)").unwrap(), TokenKind::InvalidNumeric), // malformed exponent
        //(Regex::new(r"\d+_+\b").unwrap(), TokenKind::InvalidNumeric), // trailing underscore
        //(Regex::new(r"\d+__\d+").unwrap(), TokenKind::InvalidNumeric), // trailing underscore
        
        //(Regex::new(r"\d+\.\d+n\b").unwrap(), TokenKind::InvalidBigInt), // invalid suffix
        //(Regex::new(r"\d+[eE][+-]?\d+n\b").unwrap(), TokenKind::InvalidBigInt), // invalid suffix

        //(Regex::new(r"0[xX](?:_|$)").unwrap(), TokenKind::InvalidNumeric), // invalid hex
        //(Regex::new(r"0[bB][^01]").unwrap(), TokenKind::InvalidNumeric), // invalid binary
        //(Regex::new(r"0[oO][^0-7]").unwrap(), TokenKind::InvalidNumeric), // invalid octal

        //(Regex::new(r"\d+[^0-9]").unwrap(), TokenKind::InvalidNumeric), // invalid separator placement
        
        //(Regex::new(r"\d+\.(?:\D|$)").unwrap(), TokenKind::InvalidNumeric), // Invalid float
        //(Regex::new(r"\.\d+[eE][+-]?(?:\D|$)").unwrap(), TokenKind::InvalidNumeric), // invalid exponent

        (Regex::new(r"\bnull\b").unwrap(), TokenKind::Null),
        (Regex::new(r"\b(?:true|false)\b").unwrap(), TokenKind::Boolean),
        (Regex::new(r"\bundefined\b").unwrap(), TokenKind::Undefined),

        // BigInt must come before normal numbers
        (Regex::new(r"0[xX][0-9a-fA-F]+[n]").unwrap(), TokenKind::BigInt), // hex BigInt
        (Regex::new(r"0[bB][01]+[n]").unwrap(), TokenKind::BigInt),    // binary BigInt
        (Regex::new(r"0[oO][0-7]+[n]").unwrap(), TokenKind::BigInt),   // octal BigInt
        (Regex::new(r"\d+[n]").unwrap(), TokenKind::BigInt),           // decimal BigInt

        // Numbers (without n suffix)
        (Regex::new(r"0[xX][0-9a-fA-F]+").unwrap(), TokenKind::Numeric), // hex
        (Regex::new(r"0[bB][01]+").unwrap(), TokenKind::Numeric),        // binary
        (Regex::new(r"0[oO][0-7]+").unwrap(), TokenKind::Numeric),       // octal
        (Regex::new(r"\d*\.\d+(?:[eE][+-]?\d+)?").unwrap(), TokenKind::Numeric), // float
        (Regex::new(r"\d+(?:[eE][+-]?\d+)?").unwrap(), TokenKind::Numeric),      // integer + scientific

        // Strings - single and double quotes
        (Regex::new(r#""(?:\\.|[^\\"])*""#).unwrap(), TokenKind::String),
        (Regex::new(r"'(?:\\.|[^\\'])*'").unwrap(), TokenKind::String),


        // Regex literal (very rough — this is the weak point of regex lexers)
        (Regex::new(r#"/(?:\\.|[^/\n])+/[gimsuy]*"#).unwrap(), TokenKind::RegExp),
    
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
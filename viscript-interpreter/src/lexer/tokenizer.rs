use regex::Regex;
use lazy_static::lazy_static;
use std::fmt;

use crate::lexer::{
    keyword::Keyword,
    literal:: Literal,
    token::{
        Token, TokenKind
    }
};
use crate::utility::strings::unescape_string;

#[derive(Debug)]
pub enum LexerError {
    UnexpectedCharacter(char, usize),
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::UnexpectedCharacter(c, pos) => {
                write!(f, "Unexpected character: '{}' at position {}", c, pos)
            }
        }
    }
}

impl std::error::Error for LexerError {}

// Besides Keyword, this comes from AI. May be incomplete
lazy_static! {
    static ref TOKEN_REGEX: Vec<(Regex, TokenKind)> = vec![
        // 1. Whitespace & comments (skip)
        (Regex::new(r"[ \t\r\n]+").unwrap(), TokenKind::Skip),
        (Regex::new(r"//.*").unwrap(), TokenKind::Skip),
        (Regex::new(r"/\*[\s\S]*?\*/").unwrap(), TokenKind::Skip),

        // 2. Literals - most specific first
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
    
        // Reactive Directives
        (Regex::new(r"&[A-Za-z_$][A-Za-z0-9_$]*").unwrap(), TokenKind::Directive),

        // Identifiers & keywords
        (Regex::new(r"[\p{L}_$][\p{L}\p{N}_$]*").unwrap(), TokenKind::Identifier),

        // Punctuation & operators (longest first!)
        (Regex::new(concat!(
            r"\+=|-=|\*=|/=|%=|",
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

pub fn tokenize(source_code: &str) -> Result<Vec<Token>, LexerError> {
    let mut tokens = Vec::new();
    let mut pos = 0;

    while pos < source_code.len() {
        let mut matched = false;
        
        for (regex, kind) in TOKEN_REGEX.iter() {
            if let Some(mat) = regex.find(&source_code[pos..]) {
                if mat.start() == 0 {
                    matched = true;
                    let text = mat.as_str().to_string();

                    let token = match kind {
                        TokenKind::Skip => { pos += mat.end(); break; }

                        TokenKind::Null => Token::Literal(Literal::Null),
                        TokenKind::Undefined => Token::Literal(Literal::Undefined),
                        TokenKind::Boolean => Token::Literal(Literal::Boolean(text == "true")),
                        TokenKind::BigInt => Token::Literal(Literal::BigInt(text)),
                        TokenKind::Numeric => Token::Literal(Literal::Numeric(text)),
                        TokenKind::String => Token::Literal(Literal::String(unescape_string(&text))),
                        TokenKind::RegExp => Token::Literal(Literal::RegExp(text)),

                        TokenKind::Directive => {
                            let name = text.strip_prefix('&').unwrap_or(&text).to_string();
                            Token::Directive(name)
                        },

                        TokenKind::Identifier => {
                            if let Some(keyword) = Keyword::from_str(&text) {
                                Token::Keyword(keyword)
                            } else {
                                Token::Identifier(text)
                            }
                        }
                        TokenKind::Operator => Token::Operator(text),
                        TokenKind::Punctuation => Token::Punctuation(text),
                        
                    };

                    tokens.push(token);
                    pos += mat.end();
                    break;
                }
            }
        }
        if !matched {
            return Err(LexerError::UnexpectedCharacter(
                source_code[pos..].chars().next().unwrap(), pos
            ));
        }
    }

    tokens.push(Token::EOF);
    Ok(tokens)
}
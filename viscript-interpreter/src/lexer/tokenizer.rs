use std::str::FromStr;

use crate::lexer::{
    directive::Directive,
    error::LexerError,
    keyword::Keyword,
    literal:: Literal,
    operator::Operator,
    punctuation::Punctuation,
    regex::TOKEN_REGEX,
    token::{
        Token, TokenKind
    }
};
use crate::utility::{
    span::{
        Span,
        Spanned,
        span_from_match
    },
    strings::unescape_string,
};

pub struct LexerResult {
    pub tokens: Vec<Spanned<Token>>,
    pub errors: Vec<LexerError>,
}

pub fn tokenize(source_code: &str) -> LexerResult {
    let mut tokens: Vec<Spanned<Token>> = Vec::new();
    let mut errors: Vec<LexerError> = Vec::new();
    let mut pos = 0;

    'outer: while pos < source_code.len() {
        let mut matched = false;
        
        for (regex, kind) in TOKEN_REGEX.iter() {
            if let Some(mat) = regex.find(&source_code[pos..]) {
                if mat.start() == 0 {
                    matched = true;
                    let text = mat.as_str();
                    let token = match kind {
                        TokenKind::Skip => { pos += mat.end(); break; }
                        TokenKind::InvalidNumeric | TokenKind::InvalidBigInt => {
                            errors.push(LexerError::InvalidNumber {
                                value: text.to_string(), span: span_from_match(pos, &mat)
                            });
                            pos += mat.end();
                            continue 'outer;
                        }

                        TokenKind::Null => Token::Literal(Literal::Null),
                        TokenKind::Undefined => Token::Literal(Literal::Undefined),
                        TokenKind::Boolean => Token::Literal(Literal::Boolean(text == "true")),
                        TokenKind::BigInt => Token::Literal(Literal::BigInt(text.to_string())),
                        TokenKind::Numeric => Token::Literal(Literal::Numeric(text.to_string())),
                        TokenKind::String => Token::Literal(Literal::String(unescape_string(text))),
                        TokenKind::RegExp => Token::Literal(Literal::RegExp(text.to_string())),

                        TokenKind::Directive => {
                            let name = text.strip_prefix('&').unwrap_or(text).to_string();
                            match Directive::from_str(&name) {
                                Ok(directive) => Token::Directive(directive),
                                Err(_) => {
                                    errors.push(LexerError::UnknownDirective {
                                        name: name.to_string(),
                                        span: span_from_match(pos, &mat),
                                    });

                                    pos += mat.end();
                                    continue 'outer;
                                }
                            }
                        },

                        TokenKind::Identifier | TokenKind::Keyword => {
                            if let Ok(keyword) = Keyword::from_str(text) {
                                Token::Keyword(keyword)
                            } else {
                                Token::Identifier(text.to_string())
                            }
                        }
                        TokenKind::Operator => {
                            match Operator::from_str(text) {
                                Ok(op) => Token::Operator(op),
                                Err(_) => {
                                    let ch = text.chars().next().unwrap_or('?');
                                    let error = LexerError::UnexpectedCharacter{
                                        char: ch,
                                        span: Span { start: pos, end: pos + ch.len_utf8() },
                                    };
                                    errors.push(error);
                                    pos += ch.len_utf8();
                                    continue 'outer;
                                }
                            }
                        },
                        TokenKind::Punctuation => {
                            let ch = text.chars().next().unwrap_or('\0');
                            match Punctuation::from_char(ch) {
                                Some(p) => Token::Punctuation(p),
                                None => {
                                    let ch = text.chars().next().unwrap_or('?');
                                    let error = LexerError::UnexpectedCharacter{
                                        char: ch,
                                        span: Span { start: pos, end: pos + ch.len_utf8() },
                                    };
                                    errors.push(error);
                                    pos += ch.len_utf8();
                                    continue 'outer;
                                }
                            }
                        },
                        
                    };

                    tokens.push(Spanned {
                        value: token,
                        span: span_from_match(pos, &mat),
                    });
                    pos += mat.end();
                    break;
                }
            }
        }
        if !matched {
            let ch = source_code[pos..].chars().next().unwrap();
            let error = LexerError::UnexpectedCharacter{
                char: ch,
                span: Span { start: pos, end: pos + ch.len_utf8() },
            };
            errors.push(error);
            pos += ch.len_utf8();
        }
    }

    tokens.push(Spanned {
        value: Token::EOF,
        span: Span {start: pos, end: pos},
    });
    LexerResult { tokens, errors }
}
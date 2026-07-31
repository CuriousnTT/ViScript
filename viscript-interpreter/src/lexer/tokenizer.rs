use std::str::FromStr;

use crate::lexer::{
    directive::Directive,
    error::LexerError,
    keyword::Keyword,
    literal:: Literal,
    operator::Operator,
    punctuation::Punctuation,
    regex::{
        TOKEN_REGEX,
        SKIP_REGEX,
    },
    token::{
        Token, TokenKind,
    },
};
use crate::common::strings::unescape_string;
use crate::source::{
    span::{
        Span,
        Spanned,
    },
};

pub struct LexerResult {
    pub tokens: Vec<Spanned<Token>>,
    pub errors: Vec<LexerError>,
}
pub struct Lexer<'a> {
    pub(super) source: &'a str,
    pub(super) pos: usize,
    pub(super) tokens: Vec<Spanned<Token>>,
    pub(super) errors: Vec<LexerError>,
}

impl<'a> Lexer<'a> {
    pub fn new(source:&'a str) -> Self {
        Self {
            source,
            pos: 0,
            tokens: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// tokenizes source, adding an EOF token at the end of the token list
    pub fn tokenize(mut self) -> LexerResult {
        while !self.is_eof() {
            self.next_token();
        }

        self.tokens.push(Spanned {
            value: Token::EOF,
            span: Span {
                start: self.pos,
                end: self.pos,
            },
        });

        LexerResult {
            tokens: self.tokens,
            errors: self.errors
        }
    }

    /// tokenize, but stops at until or EOF. If EOF, it adds EOF token.
    pub fn tokenize_until(mut self, until: usize) -> LexerResult {
        while !self.is_eof() {
            if self.pos >= until {
                break;
            }
            self.next_token();
        }

        if self.is_eof() {
            self.tokens.push(Spanned {
                value: Token::EOF,
                span: Span {
                    start: self.pos,
                    end: self.pos,
                },
            });
        }
        
        LexerResult {
            tokens: self.tokens,
            errors: self.errors
        }
    }

    fn next_token(&mut self) {
        let remaining = &self.source[self.pos..];

        // Skippables handled first to avoid confusion with regular expressions
        for (regex,kind) in SKIP_REGEX.iter() {
            if let Some(mat) = regex.find(remaining) {
                if mat.start() == 0 {
                    self.handle_match(mat.as_str(), mat.end(), kind);
                    return;
                }
            }
        }

        if let Some(ch) = self.peek() {
            if ch.is_ascii_digit() || (ch == '.' && self.peek_next().map(
                |c| c.is_ascii_digit()
            ).unwrap_or(false)) {
                self.scan_number();
                return;
            }
            if ch == '/' {
                if !self.tokens.last().unwrap().value.can_end_expression() {
                    self.scan_regex();
                    return;
                }
            }
            if ch == '`' {
                self.scan_template();
                return;
            }
        }

        for (regex,kind) in TOKEN_REGEX.iter() {
            if let Some(mat) = regex.find(remaining) {
                if mat.start() == 0 {
                    self.handle_match(mat.as_str(), mat.end(), kind);
                    return;
                }
            }
        }

        let ch = self.advance().unwrap();
        
        self.errors.push(
            LexerError::UnexpectedCharacter {
                char: ch,
                span: Span {
                    start: self.pos - ch.len_utf8(),
                    end: self.pos
                }
            }
        );
    }

    /// Solely for use in lexing regex-based token types
    fn handle_match(
        &mut self,
        text: &str,
        length: usize,
        kind: &TokenKind,
    ) {
        let start = self.pos;
        let end = self.pos + length;
        let span = Span { start, end };

        let token = match kind {
            TokenKind::Skip => {
                self.pos += length;
                return;
            }

            TokenKind::Null => Token::Literal(Literal::Null),
            TokenKind::Undefined => Token::Literal(Literal::Undefined),
            TokenKind::Boolean => {
                Token::Literal(Literal::Boolean(text == "true"))
            }
            TokenKind::BigInt => {
                Token::Literal(Literal::BigInt(text.to_string()))
            }
            TokenKind::Numeric => {
                Token::Literal(Literal::Numeric(text.to_string()))
            }
            TokenKind::String => {
                Token::Literal(Literal::String(unescape_string(text)))
            }
            TokenKind::RegExp => {
                Token::Literal(Literal::RegExp(text.to_string().into()))
            }
            TokenKind::Directive => {
                let name = text.strip_prefix('&').unwrap_or(text).to_string();
                match Directive::from_str(&name) {
                    Ok(directive) => Token::Directive(directive),
                    Err(_) => {
                        self.errors.push(
                            LexerError::UnknownDirective { name, span }
                        );
                        self.pos += length;
                        return;
                    }
                }
            }
            
            TokenKind::Identifier | TokenKind::Keyword => {
                if let Ok(keyword) = Keyword::from_str(text) {
                    Token::Keyword(keyword)
                } else {
                    Token::Identifier(text.to_string())
                }
            }
            TokenKind::Operator => match Operator::from_str(text) {
                Ok(op) => Token::Operator(op),
                Err(_) => {
                    let ch = text.chars().next().unwrap_or('?');
                    self.errors.push(
                        LexerError::UnexpectedCharacter { char: ch, span: span },
                    );
                    self.pos += ch.len_utf8();
                    return;
                }
            }
            TokenKind::Punctuation => {
                let ch = text.chars().next().unwrap_or('\0');
                match Punctuation::from_char(ch) {
                    Some(p) => Token::Punctuation(p),
                    None => {
                        self.errors.push(LexerError::UnexpectedCharacter { char: ch, span });
                        self.pos += ch.len_utf8();
                        return;
                    }
                }
            }
        };

        self.tokens.push(Spanned { value: token, span: span});
        self.pos += length;
    }

    pub(super) fn peek(&self) -> Option<char> {
        self.source[self.pos..].chars().next()
    }

    pub(super) fn peek_next(&self) -> Option<char> {
        self.source[self.pos..].chars().nth(1)
    }

    pub(super) fn peek_n(&self, n: usize) -> Option<char> {
        self.source[self.pos..].chars().nth(n)
    }

    pub(super) fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.pos += ch.len_utf8();
        Some(ch)
    }

    pub(super) fn is_eof(&self) -> bool {
        self.pos >= self.source.len()
    }
}

/// Creates a Lexer for the source_code and uses its tokenize method
pub fn tokenize(source_code: &str) -> LexerResult {
    Lexer::new(source_code).tokenize()
}

/// Creates a Lexer for the source_code, tokenizing the range from start to stop.
pub fn tokenize_range(source_code: &str, start: usize, stop: usize) -> LexerResult {
    let mut lexer = Lexer::new(source_code);
    lexer.pos = start;
    lexer.tokenize_until(stop)
}
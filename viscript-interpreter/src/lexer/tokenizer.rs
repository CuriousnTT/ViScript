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
        Token, TokenKind,
    }
};
use crate::utility::{
    span::{
        Span,
        Spanned,
    },
    strings::unescape_string,
};

pub struct LexerResult {
    pub tokens: Vec<Spanned<Token>>,
    pub errors: Vec<LexerError>,
}
pub struct Lexer<'a> {
    source: &'a str,
    pos: usize,
    tokens: Vec<Spanned<Token>>,
    errors: Vec<LexerError>,
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

    fn next_token(&mut self) {
        let remaining = &self.source[self.pos..];

        if let Some(ch) = self.peek() {
            if ch.is_ascii_digit() || (ch == '.' && self.peek_next().map(
                |c| c.is_ascii_digit()
            ).unwrap_or(false)) {
                self.scan_number();
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
                Token::Literal(Literal::RegExp(text.to_string()))
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

    fn peek(&self) -> Option<char> {
        self.source[self.pos..].chars().next()
    }

    fn peek_next(&self) -> Option<char> {
        self.source[self.pos..].chars().nth(1)
    }

    fn peek_n(&self, n: usize) -> Option<char> {
        self.source[self.pos..].chars().nth(n)
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.pos += ch.len_utf8();
        Some(ch)
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.source.len()
    }

    fn scan_number(&mut self) {
        let start = self.pos;
        let mut saw_n = false;
        let mut saw_dot = false;
        let mut saw_exponent = false;

        let radix = 
            if let Some('0') = self.peek() {
                match self.peek_next() {
                    Some('x' | 'X') => {
                        self.advance(); // 0
                        self.advance(); // x
                        16
                    }
                    Some('b' | 'B') => {
                        self.advance(); // 0
                        self.advance(); // b
                        2
                    }
                    Some('o' | 'O') => {
                        self.advance();
                        self.advance(); // o
                        8
                    }
                    _ => 10
                }
            } else {
                10
            };

        while let Some(ch) = self.peek() {
            match ch {
                ch if Self::valid_digit(ch, radix) || ch == '_' => {
                    self.advance();
                }
                '.' if radix == 10 && !saw_dot && !saw_exponent => {
                    saw_dot = true;
                    self.advance();
                }
                'e' | 'E' if radix == 10 && !saw_exponent => {
                    // Determine if this is an exponent or the next token
                    let next = self.peek_next();
                    let next2 = self.peek_n(2);

                    let valid_exponent =
                        matches!(next, Some('0'..='9')) ||
                        matches!(next, Some('+' | '-')) &&
                        matches!(next2, Some('0'..='9'));
                    
                    if valid_exponent {
                        saw_exponent = true;
                        self.advance(); // consumes e

                        if  matches!(self.peek(), Some('+' | '-')) {
                            self.advance(); // consumes +/-
                        }
                    } else {
                        break; // e signified start of a new token
                    }

                    

                }
                'n' => {
                    saw_n = true;
                    self.advance();
                    break; // must be the end of a BigInt
                }
                // stop at any character that cannot continue a number
                _ => {  break; }
            }
        }

        let end = self.pos;
        let text = &self.source[start..end];

        let kind = if saw_n {
            TokenKind::BigInt
        } else {
            TokenKind::Numeric
        };

        self.validate_number(text, start, end);

        let token = if saw_n {
            Token::Literal(Literal::BigInt(text.to_string()))
        } else {
            Token::Literal(Literal::Numeric(text.to_string()))
        };

        self.tokens.push(Spanned {
            value: token,
            span: Span {start, end },
        });
    }

    fn validate_number(
        &mut self,
        text: &str,
        start: usize,
        end: usize
    ) {
        let span = Span { start, end };
        let (radix, body) = 
            if text.starts_with("0x") || text.starts_with("0X") {
                (16, &text[2..])
            } else if text.starts_with("0b") || text.starts_with("0B") {
                (2, &text[2..])
            } else if text.starts_with("0o") || text.starts_with("0O") {
                (8, &text[2..])
            } else {
                (10, text)
            };
        
        let can_have_exponent = radix == 10;

        if text.ends_with('n') {
            let bigint_err = LexerError::InvalidBigInt {
                    value: text.to_string(),
                    span,
                };
            
            let without_n = &body[..body.len()-1];
            if without_n.is_empty() || without_n.contains('.') || without_n.contains('-') || without_n.contains('+') {
                self.errors.push(bigint_err);
                return;
            } else if radix != 16 && // e is only legal in BigInt for radix 16
                (without_n.contains('e') || without_n.contains('E')) {
                self.errors.push(bigint_err);
                return;
            }

            if !Self::validate_digits_and_separators(without_n, radix) {
                self.errors.push(bigint_err);
                return;
            }

            return;

        } else {
            let number_err = LexerError::InvalidNumber { value: text.to_string(), span };
            let can_have_point = radix == 10;
            
            if body.is_empty() {
                self.errors.push(number_err);
                return;
            }

            if !can_have_exponent && (body.contains('e') || body.contains('E')) {
                self.errors.push(number_err);
                return;
            }

            if !can_have_point && body.contains('.') {
                self.errors.push(number_err);
                return;
            }

            if radix == 10 {
                if !Self::validate_decimal_literal(body) {
                    self.errors.push(number_err);
                    return;
                }
            } else {
                if !Self::validate_digits_and_separators(body, radix) {
                    self.errors.push(number_err);
                    return;
                }
            }
        }
    }

    fn valid_digit(ch: char, radix: u32) -> bool {
        match radix {
            2=> matches!(ch, '0' | '1'),
            8 => matches!(ch, '0'..='7'),
            10 => matches!(ch, '0'..='9'),
            16 => ch.is_ascii_hexdigit(),
            _ => false,
        }
    }

    fn validate_digits_and_separators(text: &str, radix: u32) -> bool {
        let bytes = text.as_bytes();

        if text.is_empty() {
            return false;
        }

        for i in 0..bytes.len() {
            match bytes[i] {
                b'_' => {
                    if i == 0 || i + 1 == bytes.len() {
                        return false;
                    }

                    let prev = bytes[i - 1] as char;
                    let next = bytes[i + 1] as char;
                    
                    if !Self::valid_digit(prev,radix) || !Self::valid_digit(next, radix) {
                        return false;
                    }

                }
                b =>{
                    if !Self::valid_digit(b as char, radix) {
                        return false;
                    }
                } 
            }
        }
        true
    }

    fn validate_decimal_literal(text: &str) -> bool {
        let (mantissa, exponent) = 
            match text.find(['e', 'E']) {
                Some(idx) => (&text[..idx], Some(&text[idx+1..])),
                None => (text, None),
            };
        
        if let Some(exp) = exponent {
            let exp = 
                if exp.starts_with('+') || exp.starts_with('-') {
                    &exp[1..]
                } else {
                    exp
                };
            
            if !Self::validate_digits_and_separators(exp, 10) {
                return false;
            }
        }

        let parts: Vec<_> = mantissa.split('.').collect();

        if parts.len() > 2 {
            return false;
        } else if parts.len() == 1 {
            return Self::validate_digits_and_separators(parts[0], 10);
        } else if parts.len() == 2 {
            let integer = parts[0];
            let fraction = parts[1];

            if integer.is_empty() && fraction.is_empty() {
                return false;
            }

            let integer_valid_or_empty = integer.is_empty() || Self::validate_digits_and_separators(integer, 10);
            let fraction_valid_or_empty = fraction.is_empty() || Self::validate_digits_and_separators(fraction, 10);

            return integer_valid_or_empty && fraction_valid_or_empty;

        } else {
            return false;
        }
    }
}

pub fn tokenize(source_code: &str) -> LexerResult {
    Lexer::new(source_code).tokenize()
}
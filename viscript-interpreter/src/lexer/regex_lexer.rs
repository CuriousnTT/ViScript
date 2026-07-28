use std::collections::HashSet;

use crate::lexer::{
    error::LexerError,
    literal::Literal,
    token::Token,
    tokenizer::Lexer,
};
use crate::source::{
    span::{
        Span,
        Spanned,
    },
};

struct RegexBody {
    body_end: usize, // position of closing candidate closing '/'
    first_newline: Option<usize>,
}

struct RegexFlags {
    flags: String,
    flags_end: usize,
    duplicate_flags: HashSet<char>,
    invalid_flags: HashSet<char>,
}

impl <'a> Lexer <'a> {
    pub(super) fn scan_regex(&mut self) {
        let start = self.pos;

        self.advance(); // opening '/'

        let body = self.scan_regex_body();
        let flags = self.scan_regex_flags();

        self.validate_regex(start, body, flags);
    }

    fn scan_regex_body(&mut self) -> RegexBody {
        let mut escaped = false;
        let mut in_class = false;

        let mut first_newline = None;

        loop {
            match self.peek() {

                None => {
                    return RegexBody {
                        body_end: self.pos,
                        first_newline,
                    };
                }

                Some(ch) => {
                    if escaped {
                        escaped = false;
                    } else {

                        match ch {
                            '\\' => escaped = true,

                            '[' if !in_class => in_class = true,

                            ']' if in_class => in_class = false,

                            '/' if !in_class => {
                                let slash = self.pos;
                                self.advance();
                                return RegexBody {
                                    body_end: slash,
                                    first_newline,
                                };
                            }

                            '\n' | '\r' => {
                                first_newline.get_or_insert(self.pos);
                            }

                            _ => {}
                        }
                    }

                    self.advance();
                }
            }
        }
    }

    fn scan_regex_flags(&mut self) -> RegexFlags {
        let start = self.pos;
        let mut flags = String::new();
        let mut duplicate: HashSet<char> = HashSet::new();
        let mut invalid: HashSet<char> = HashSet::new();

        const VALID: &[char] = &[
            'd','g','i','m','s','u','v','x','y'
        ];

        while let Some(ch) = self.peek() {
            if !ch.is_ascii_alphabetic() {
                break;
            }
            if flags.contains(ch) {
                duplicate.insert(ch);
            };

            if !VALID.contains(&ch) {
                invalid.insert(ch);
            }
            
            flags.push(ch);
        
            self.advance();
        }

        RegexFlags {
            flags,
            flags_end: self.pos,
            duplicate_flags: duplicate,
            invalid_flags: invalid,
        }
    }

    fn validate_regex(&mut self, start: usize, body: RegexBody, flags: RegexFlags) {
        // if x-flag, multiline is allowed, otherwise only the first line is regex -> must be handled first
        let has_x = flags.flags.contains('x');
        let has_newline = body.first_newline.is_some();

        if has_newline && !has_x {
            // Unterminated regex
            let end = body.first_newline.unwrap(); // first \n or \r
            
            let span = Span { start, end };
            let text = &self.source[start..end];

            self.errors.push(
                LexerError::UnterminatedRegex { text: text.to_string(), span }
            );

            self.tokens.push(Spanned {
                value: Token::Literal(Literal::RegExp(text.to_string().into())),
                span: span,
            });

            self.pos = end; // Rewind so other tokens can be lexed.
            return;
        }

        let end = flags.flags_end;
        let span = Span { start, end };
        let text = &self.source[start..end];
        if !flags.duplicate_flags.is_empty() || !flags.invalid_flags.is_empty() {
            self.errors.push(LexerError::InvalidRegExp { value: text.to_string(), span });
        }

        self.tokens.push(Spanned {
            value: Token::Literal(Literal::RegExp(text.to_string().into())),
            span
        });

    }

    pub(super) fn can_end_expression(token: &Token) -> bool {
        token.can_end_expression()
    }
}
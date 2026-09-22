use std::str::FromStr;

use crate::lexer::{
    contexts::{
        BraceContext,
        ParenContext,
    },
    error::LexerError,
    regex::{
        SKIP_REGEX, TOKEN_REGEX,
    },

};
use crate::common::strings::unescape_string;
use crate::source::{
    span::{
        Span,
        Spanned,
    },
};
use crate::token::{
    directive::Directive,
    keyword::Keyword,
    literal:: Literal,
    operator::Operator,
    punctuation::Punctuation,
    token::{
        Token,
        TokenKind,
    },
};

pub struct LexerResult {
    pub tokens: Vec<Spanned<Token>>,
    pub errors: Vec<LexerError>,
}
pub struct Lexer<'a> {
    pub(super) source: &'a str,
    pub(super) pos: usize,
    pub(super) expr_allowed: bool,
    pub(super) vsx_enabled: bool,
    
    paren_stack: Vec<ParenContext>,
    brace_stack: Vec<BraceContext>,

    pub(super) tokens: Vec<Spanned<Token>>,
    pub(super) errors: Vec<LexerError>,
}

impl<'a> Lexer<'a> {
    /// vsx is disabled by default
    pub fn new(source: &'a str) -> Self {
        Self::new_with_vsx(source, false)
    }
    pub fn new_with_vsx(source:&'a str, vsx_enabled: bool) -> Self {
        Self {
            source,
            pos: 0,
            expr_allowed: true,
            vsx_enabled: vsx_enabled,
            paren_stack: Vec::new(),
            brace_stack: Vec::new(),
            tokens: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// tokenizes source, adding an EOF token at the end of the token list
    pub fn tokenize(mut self) -> LexerResult {
        while !self.is_eof() {
            self.next_token();
        }

        self.push_token(
            Token::EOF,
            Span::empty(self.pos)
        );

        LexerResult {
            tokens: self.tokens,
            errors: self.errors
        }
    }

    /// tokenize, but stops at until or EOF. Does *NOT* add EOF token, even for EOF.
    /// 'until' is mostly symbolic: The idiomatic way to call this method is through tokenize_range.
    /// This is because token-lexing methods may exceed 'until' before returning to the main loop,
    /// resulting in the method overshooting its target.
    /// This is not a risk with tokenize_range, which makes a Lexer that doesn't see beyond 'until'.
    pub fn tokenize_until(mut self, until: usize) -> LexerResult {
        while !self.is_eof() {
            if self.pos >= until {
                break;
            }
            self.next_token();
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
            if ch == '/' && self.should_scan_regex() {
                self.scan_regex();
                return;
            }
            if self.vsx_enabled && ch == '<' {
                if self.should_scan_vsx() { // handled after vsx_enabled for being heavier
                    self.scan_vsx();
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
            TokenKind::String => {
                Token::Literal(Literal::String(unescape_string(text)))
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
            TokenKind::Punctuation => match Punctuation::from_str(text) {
                Some(p) => Token::Punctuation(p),
                None => {
                    let ch = text.chars().next().unwrap_or('?');
                    self.errors.push(
                        LexerError::UnexpectedCharacter { char: ch, span }
                    );
                    self.pos += ch.len_utf8();
                    return;
                }
            }
        };

        self.push_token(token, span);
        self.pos += length;
    }

    /// method for pushing tokens while updating lexer state
    /// should be used in all cases that don't rely on nested lexers
    pub(super) fn push_token(&mut self, token:Token, span: Span) {
        self.update_context(&token);
        self.tokens.push(Spanned {
            value: token,
            span,
        })
    }

    pub(super) fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.pos += ch.len_utf8();
        Some(ch)
    }

    pub(super) fn is_eof(&self) -> bool {
        self.pos >= self.source.len()
    }

    fn update_context(&mut self, token: &Token) {
        // 1. Always update the simple expr_allowed flag first
        //    (covers identifiers, literals, most operators, keywords, …)
        self.update_expr_allowed(token);

        // 2. Maintain the stacks (and override expr_allowed when a close happens)
        match token {
            // ---------- parentheses ----------
            Token::Punctuation(Punctuation::LeftParen) => {
                // Decide which kind of paren this is.
                // Look at the *previous* token (already pushed) to recognise
                // if / while / for / with / catch.
                let ctx = match self.tokens.last().map(|t| &t.value) {
                    Some(Token::Keyword(Keyword::If))    => ParenContext::If,
                    Some(Token::Keyword(Keyword::While)) => ParenContext::While,
                    Some(Token::Keyword(Keyword::For))   => ParenContext::For,
                    Some(Token::Keyword(Keyword::With))  => ParenContext::With,
                    Some(Token::Keyword(Keyword::Catch)) => ParenContext::Catch,
                    _ => ParenContext::Expression,
                };
                self.paren_stack.push(ctx);
                // Opening paren itself always allows a regex afterwards
                // (already set by update_expr_allowed)
            }

            Token::Punctuation(Punctuation::RightParen) => {
                if let Some(ctx) = self.paren_stack.pop() {
                    // The decisive part:
                    self.expr_allowed = match ctx {
                        ParenContext::Expression => false, // ends a PrimaryExpression
                        // control-flow parens end a condition -> statement can start
                        ParenContext::If
                        | ParenContext::While
                        | ParenContext::For
                        | ParenContext::With
                        | ParenContext::Catch => true,
                    };
                } else {
                    // Unmatched `)` – treat as expression-style for safety
                    self.expr_allowed = false;
                }
            }

            // ---------- braces ----------
            Token::Punctuation(Punctuation::LeftBrace) => {
                // Heuristic: if a regex would currently be *disallowed*,
                // we are almost certainly inside an expression context → object.
                // Otherwise it is a block.
                let ctx = if self.expr_allowed {
                    BraceContext::Block
                } else {
                    BraceContext::Object
                };
                self.brace_stack.push(ctx);
                // Opening brace itself always allows a regex (already set)
            }

            Token::Punctuation(Punctuation::RightBrace) => {
                if let Some(ctx) = self.brace_stack.pop() {
                    self.expr_allowed = match ctx {
                        BraceContext::Block  => true,  // new statement can start
                        BraceContext::Object => false, // ends a PrimaryExpression
                    };
                } else {
                    // Unmatched `}` – treat as block-style
                    self.expr_allowed = true;
                }
            }

            // Everything else is already handled by update_expr_allowed
            _ => {}
        }
    }

    fn update_expr_allowed(&mut self, token: &Token) {
        use Keyword::*;
        use Operator::*;
        use Punctuation::*;

        self.expr_allowed = match token {
            // Primary expressions → regex forbidden
            Token::Identifier(_) | Token::Literal(_) => false,

            // Keywords that introduce an expression / statement that can be followed by /
            Token::Keyword(Return | Throw | Delete | Void | Typeof | New | Yield | Await) => true,

            // Most operators allow a regex (they expect an expression)
            Token::Operator(op) => match op {
                Increment | Decrement => false, // postfix
                _ => true,
            },

            // Opening punctuation always allows a regex
            Token::Punctuation(LeftParen | LeftBrace | LeftBracket | Comma | Colon | Semicolon) => true,

            // Closing punctuation – *do not decide here*; the stack will override
            Token::Punctuation(RightParen | RightBrace | RightBracket) => {
                // temporary value; will be replaced by update_context
                false
            }

            // Everything else (directives, EOF, …) – keep previous state
            _ => self.expr_allowed,
        };
    }
}

/// Creates a Lexer for the source_code and uses its tokenize method
pub fn tokenize(source_code: &str, vsx_enabled: bool) -> LexerResult {
    Lexer::new_with_vsx(source_code, vsx_enabled).tokenize()
}

/// Creates a Lexer for the source_code, tokenizing the range from start to stop.
pub fn tokenize_range(
    source_code: &str, vsx_enabled: bool, start: usize, stop: usize
) -> LexerResult {
    let source = &source_code[..stop];
    let mut lexer = Lexer::new_with_vsx(source, vsx_enabled);
    lexer.pos = start;
    lexer.tokenize_until(stop)
}
use crate::token::token::Token;
use crate::source::span::{
    Span,
    Spanned,
};

pub struct TokenCursor<'a> {
    tokens: &'a [Spanned<Token>],
    position: usize,
}

impl<'a> TokenCursor<'a> {
    pub fn new(tokens: &'a [Spanned<Token>]) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    /// Returns the current token without consuming it.
    pub fn current(&self) -> Option<&'a Spanned<Token>> {
        self.tokens.get(self.position)
    }

    /// Returns the token `offset` positions after the current token.
    /// 
    /// `peek(0)` is equivalent to `current()`.
    pub fn peek(&self, offset: usize) -> Option<&'a Spanned<Token>> {
        self.tokens.get(self.position.checked_add(offset)?)
    }

    /// Consumes and returns the current token.
    pub fn advance(&mut self) -> Option<&'a Spanned<Token>> {
        let token = self.tokens.get(self.position)?;
        self.position += 1;
        Some(token)
    }

    /// Returns true if the cursor has moved past the final token.
    pub fn is_exhausted(&self) -> bool {
        self.position >= self.tokens.len()
    }

    /// Returns the span of the current token,
    /// Or an empty span for the end if exhausted.
    pub fn current_span(&self) -> Span {
        match self.current() {
            Some(token) => token.span,

            None => self.tokens.last().map(
                |token| Span::empty(
                    token.span.end
                )
            ).unwrap_or(
                Span::empty(0)
            )
        }
    }

    /// Returns the current position in the underlying token stream.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Sets the cursor to a position.
    pub fn restore(&mut self, position: usize) {
        debug_assert!(position <= self.tokens.len());
        self.position = position;
    }
}
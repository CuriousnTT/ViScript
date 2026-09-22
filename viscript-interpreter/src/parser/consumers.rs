use crate::ast::directive::Directive;
use crate::parser::parser::Parser;
use crate::source::span::Spanned;
use crate::token::{
    keyword::Keyword,
    operator::Operator,
    punctuation::Punctuation,
    token::Token,
};

impl<'a> Parser<'a> {
    // Consumes -> Optional consumption if present

    /// Generic consume helper
    pub(super) fn consume_token<F, T>(
        &mut self, extract: F
    ) -> Option<Spanned<T>>
        where F: FnOnce(&Token) -> Option<T> {
        let token = self.current()?;

        let value = extract(&token.value)?;
        let span = token.span;

        self.advance();
        Some(Spanned { value, span })
    }

    /// consumes keyword if present
    pub(super) fn consume_keyword(&mut self, keyword: Keyword) -> Option<Spanned<Keyword>> {
        self.consume_token(|token| {
            match token {
                Token::Keyword(k) if *k == keyword => Some(*k),
                _ => None,
            }
        })
    }

    /// consumes one of the given keywords if present.
    pub(super) fn consume_keyword_from_slice(&mut self, keywords: &[Keyword]) -> Option<Spanned<Keyword>> {
        self.consume_token(|token| match token {
            Token::Keyword(k) if keywords.contains(k) => Some(*k),
            _ => None,
        })
    }

    /// consumes operator if present
    pub(super) fn consume_operator(&mut self, operator: Operator) -> Option<Spanned<Operator>> {
        self.consume_token(|token| {
            match token {
                Token::Operator(o) if *o == operator => Some(*o),
                _ => None,
            }
        })
    }

    /// consumes one of the given operators if present.
    pub(super) fn consume_operator_from_slice(&mut self, operators: &[Operator]) -> Option<Spanned<Operator>> {
        self.consume_token(|token| {
            match token {
                Token::Operator(o) if operators.contains(o) => Some(*o),
                _ => None,
            }
        })
    }

    /// consumes punctuation if present
    pub(super) fn consume_punctuation(&mut self, punctuation: Punctuation) -> Option<Spanned<Punctuation>> {
        self.consume_token(|token| {
            match token {
                Token::Punctuation(p) if *p == punctuation => Some(*p),
                _ => None,
            }
        })
    }

    /// consumes one of the given punctuations if present.
    pub(super) fn consume_punctuation_from_slice(&mut self, punctuations: &[Punctuation]) -> Option<Spanned<Punctuation>> {
        self.consume_token(|token| {
            match token {
                Token::Punctuation(p) if punctuations.contains(p) => Some(*p),
                _ => None,
            }
        })
    }

    /// consumes a directive if present (no arguments needed)
    pub(super) fn consume_directive(&mut self) -> Option<Spanned<Directive>> {
        self.consume_token(|token| {
            match token {
                Token::Directive(d) => Some(*d),
                _ => None,
            }
        })
    }

    /// consumes all directives that are present (no arguments needed)
    pub(super) fn consume_directives(&mut self) -> Vec<Spanned<Directive>> {
        let mut directives = Vec::new();

        while let Some(directive) = self.consume_directive() {
            directives.push(directive)
        }

        directives
    }
}
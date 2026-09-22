use std::mem::take;

use crate::ast::{
    statement_list::{
        Program,
        Block,
    },
    statement::Statement,
};
use crate::parser::cursor::TokenCursor;
use crate::parser::error::ParserError;
use crate::source::span::Spanned;
use crate::token::{
    punctuation::Punctuation,
    token::Token,
};

/// semantics:
/// consume -> move past optional syntax,
/// expect -> move past required syntax,
/// parse -> move past while extracting value,
pub struct Parser<'a> {
    pub(super) cursor: TokenCursor<'a>,
    pub(super) errors: Vec<ParserError>,
}

pub struct ParseResult {
    pub ast: Option<Program>,
    pub errors: Vec<ParserError>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Spanned<Token>]) -> Self {
        Self {
            cursor: TokenCursor::new(tokens),
            errors: Vec::new(),
        }
    }

    pub fn parse_program(&mut self) -> ParseResult {
        let mut statements: Vec<Spanned<Statement>> = Vec::new();

        while !self.current().is_some_and(|token| token.value.is_eof()) {
            match self.parse_statement() {
                Ok(statement) => {
                    statements.push(statement);
                }

                Err(error) => {
                    self.errors.push(error);

                    self.recover_statement();
                }
            }
        }

        let span = statements.first().unwrap().span.until(
            statements.last().unwrap().span
        );
        ParseResult {
            ast: Some(Program {
                statements,
                span
            }),
            errors: take(&mut self.errors)
        }
    }

    pub(super) fn parse_block(&mut self) -> Result<Block, ParserError> {
        let opening = self.expect_punctuation(Punctuation::LeftBrace)?;

        let mut statements: Vec<Spanned<Statement>> = Vec::new();

        while !self.current().is_some_and(|token| {
            matches!(
                token.value,
                Token::Punctuation(Punctuation::RightBrace)
            )
        }) {
            statements.push(self.parse_statement()?);
        }

        let closing = self.expect_punctuation(Punctuation::RightBrace)?;

        Ok(Block {
            statements,
            span: opening.span.until(closing.span),
        })
    }

    // Parse -> Value extraction
    pub(super) fn parse_identifier(&mut self) -> Result<Spanned<String>, ParserError> {
        let Some(token) = self.current() else {
            return Err(ParserError::UnexpectedEof {
                span: self.cursor.current_span(),
            })
        };

        match &token.value {
            Token::Identifier(name) => {
                let identifier = Spanned {
                    value: name.clone(),
                    span: token.span,
                };

                self.advance();

                Ok(identifier)
            }
            _ => Err(ParserError::UnexpectedToken {
                expected: vec![Token::Identifier(String::new())],
                found: token.clone(), 
            }),
        }
    }

    /// Currently quite stupid, but at least it is *some* error recovery
    fn recover_statement(&mut self) {
        while let Some(token) = self.current() {
            if token.value.is_eof() {
                return;
            }

            if token.value == Token::Punctuation(Punctuation::Semicolon) {
                self.advance();
                return;
            }

            self.advance();
        }
    }

    // Cursor access
    /// extends cursor.current()
    pub(super) fn current(&self) -> Option<&'a Spanned<Token>> {
        self.cursor.current()
    }

    /// extends cursor.peek()
    pub(super) fn peek(&self, offset: usize) -> Option<&'a Spanned<Token>> {
        self.cursor.peek(offset)
    }

    /// extends cursor.advance()
    pub(super) fn advance(&mut self) -> Option<&'a Spanned<Token>> {
        self.cursor.advance()
    }
}

/// Create a Parser and use it to parse a slice of tokens as an AST
pub fn parse(tokens: &[Spanned<Token>]) -> ParseResult {
    let mut parser = Parser::new(tokens);
    parser.parse_program()
}

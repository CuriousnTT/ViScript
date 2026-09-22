use crate::ast::{
    statement::Statement,
    tuple::Tuple,
    tuple_position::TuplePosition,
};
use crate::parser::error::ParserError;
use crate::parser::parser::Parser;
use crate::source::span::Spanned;
use crate::token::{
    punctuation::Punctuation,
    token::Token,
};

impl<'a> Parser<'a> {
    pub(super) fn parse_tuple(&mut self) -> Result<Spanned<Tuple>, ParserError> {
        let opening = self.expect_punctuation(Punctuation::LeftParen)?;

        let mut positions: Vec<Spanned<TuplePosition>> = Vec::new();

        if let Some(closing) = self.consume_punctuation(Punctuation::RightParen) {
            return Ok(Spanned {
                value: Tuple { positions },
                span: opening.span.until(closing.span),
            })
        };

        loop {
            positions.push(self.parse_tuple_position()?);

            if self.consume_punctuation(Punctuation::Comma).is_some() {
                continue;
            }

            let closing = self.expect_punctuation(Punctuation::RightParen)?;

            return Ok(Spanned {
                value: Tuple { positions },
                span: opening.span.until(closing.span),
            })
        }
    }

    fn parse_tuple_position(&mut self) -> Result<Spanned<TuplePosition>, ParserError> {
        let mut statements: Vec<Spanned<Statement>> = Vec::new();
        let mut last_expression_error: Option<ParserError> = None;

        loop {
            let start = self.cursor.position();

            let label = if self.current_matches_tuple_label() {
                let label = self.parse_identifier()?;
                self.expect_punctuation(Punctuation::Colon)?;
                Some(label)
            } else {
                None
            };

            match self.parse_expression() {
                Ok(value) => {
                    if self.current_ends_tuple_position() {
                        // success, build return
                        let start_span = if let Some(statement) = statements.first() {
                            statement.span
                        } else if let Some(label) = &label {
                            label.span
                        } else {
                            value.span
                        };
                        let span = start_span.until(value.span);
                        return Ok(Spanned {
                            value: TuplePosition {
                                statements,
                                value,
                                label
                            },
                            span,
                        })
                    }

                }
                Err(error) => {
                    last_expression_error = Some(error);
                }
            }

            self.cursor.restore(start);

            // try to consume one statement
            match self.parse_statement() {
                Ok(statement) => {
                    statements.push(statement);
                },
                Err(statement_error) => return Err(statement_error)
            };
        }
    }

    // convenience helpers
    fn current_ends_tuple_position(&self) -> bool {
        matches!(
            self.current().map(|token| &token.value),
            Some(Token::Punctuation(Punctuation::Comma))
            | Some(Token::Punctuation(Punctuation::RightParen))
        )
    }

    /// tuple position labels consist of identifier + colon
    pub(super) fn current_matches_tuple_label(&self) -> bool {
        matches!(
            (self.peek(0), self.peek(1)),
            (
                Some(Spanned {
                    value: Token::Identifier(_),
                    ..
                }),
                Some(Spanned {
                    value: Token::Punctuation(Punctuation::Colon),
                    ..
                }),
            )
        )
    }
}
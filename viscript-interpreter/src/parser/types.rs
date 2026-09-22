use crate::ast::{
    tuple_type::TupleType,
    tuple_type_position::{
        TupleTypePosition,
        TupleRepetition,
    },
    type_expression::TypeExpression,
};
use crate::parser::error::ParserError;
use crate::parser::parser::Parser;
use crate::source::span::Spanned;
use crate::token::{
    literal::Literal,
    operator::Operator,
    punctuation::Punctuation,
    token::Token,
};

impl<'a> Parser<'a> {
    pub(super) fn parse_type_expression(
        &mut self,
    ) -> Result<Spanned<TypeExpression>, ParserError> {
        let expression = self.parse_primary_type_expression()?;

        let span = expression.span;

        Ok(Spanned {
            value: expression.value,
            span
        })
    }

    fn parse_primary_type_expression(
        &mut self
    ) -> Result<Spanned<TypeExpression>, ParserError> {
        let Some(token) = self.current() else {
            return Err(ParserError::UnexpectedEof {
                span: self.cursor.current_span(),
            })
        };

        match &token.value {
            Token::Identifier(name) => {
                self.advance();

                Ok(Spanned {
                    value: TypeExpression::Identifier(name.clone()),
                    span: token.span,
                })
            }

            Token::Punctuation(Punctuation::LeftParen) => {
                let tuple_type = self.parse_tuple_type()?;
                let span = tuple_type.span;

                Ok(Spanned {
                    value: TypeExpression::Tuple(tuple_type),
                    span
                })
            }

            _ => panic!("TODO")

        }
    }

    fn parse_tuple_type(
        &mut self,
    ) -> Result<Spanned<TupleType>, ParserError> {
        let opening = self.expect_punctuation(Punctuation::LeftParen)?;

        let mut positions: Vec<Spanned<TupleTypePosition>> = Vec::new();

        if let Some(closing) = self.consume_punctuation(Punctuation::RightParen) {
            return Ok(Spanned {
                value: TupleType { positions },
                span: opening.span.until(closing.span),
            })
        };

        loop {
            positions.push(self.parse_tuple_type_position()?);

            if self.consume_punctuation(Punctuation::Comma).is_some() {
                continue;
            }

            let closing = self.expect_punctuation(Punctuation::RightParen)?;

            return Ok(Spanned {
                value: TupleType { positions },
                span: opening.span.until(closing.span),
            })
        }

    }

    fn parse_tuple_type_position(
        &mut self,
    ) -> Result<Spanned<TupleTypePosition>, ParserError> {
        
        if self.current_matches_tuple_label() {
            let label = self.parse_identifier()?;
            self.expect_punctuation(Punctuation::Colon)?;

            let type_ = self.parse_type_expression()?;

            let span = label.span.until(type_.span);

            return Ok(Spanned {
                value: TupleTypePosition::Labeled {
                    type_,
                    label
                },
                span,
            })
        }

        if self.current_matches_unclear_tuple_type_position() {
            let identifier = self.parse_identifier()?;

            let span = identifier.span;

            return Ok(Spanned {
                value: TupleTypePosition::Unclear { identifier },
                span,
            })
        }

        let type_ = self.parse_type_expression()?;

        let repetition = self.parse_tuple_type_repetition()?;

        let span = match &repetition {
            Some(repetition) => type_.span.until(repetition.value.span),
            None => type_.span,
        };

        Ok(Spanned {
            span,
            value: TupleTypePosition::Unlabeled {
                type_,
                repetition
            },
        })
    }

    fn parse_tuple_type_repetition(
        &mut self,
    ) -> Result<Option<TupleRepetition>, ParserError> {
        if self.consume_operator(
            Operator::Multiplication
        ).is_none() {
            return Ok(None);
        }

        let Some(token) = self.current() else {
            return Err(ParserError::UnexpectedEof {
                span: self.cursor.current_span(),
            })
        };

        match &token.value {
            Token::Literal(Literal::Numeric(value)) => {
                self.advance();

                Ok(Some(TupleRepetition {
                    value: Spanned {
                        value: value.clone(),
                        span: token.span
                    },
                }))
            }

            _ => {
                // proper parser error eventually
                panic!("TODO")
            }
        }
    }

    /// an unclear Tuple ***Type*** Position consists of a single identifier
    fn current_matches_unclear_tuple_type_position(&self) -> bool {
        matches!(
            (self.peek(0), self.peek(1)),
            (
                Some(Spanned {
                    value: Token::Identifier(_),
                    ..
                }),
                Some(Spanned {
                    value: Token::Punctuation(
                        Punctuation::Comma | Punctuation::RightParen
                    ),
                    ..
                }),
            )
        )
    }
}

use crate::ast::expression::Expression;
use crate::parser::error::ParserError;
use crate::parser::parser::Parser;
use crate::source::span::Spanned;
use crate::token::{
    literal::Literal,
    punctuation::Punctuation,
    token::Token,
};

impl<'a> Parser<'a> {
    pub(super) fn parse_expression(& mut self) -> Result<Spanned<Expression>, ParserError> {
        let mut expression = self.parse_primary_expression()?;

        expression = self.parse_expression_postfix(expression)?;

        expression = self.parse_expression_binary(expression, 0)?;

        Ok(expression)
    }

    fn parse_primary_expression(&mut self) -> Result<Spanned<Expression>, ParserError> {
        let Some(token) = self.current() else {
            return Err(ParserError::UnexpectedEof {
                span: self.cursor.current_span(),
            })
        };

        match &token.value {
            Token::Literal(Literal::Numeric(value)) => {
                self.advance();

                Ok(Spanned {
                    value: Expression::NumericLiteral(value.clone()),
                    span: token.span,
                })
            }

            Token::Literal(Literal::String(value)) => {
                self.advance();
                Ok(Spanned {
                    value: Expression::StringLiteral(value.clone()),
                    span: token.span,
                })
            }

            Token::Literal(Literal::Null) => {
                self.advance();
                Ok(Spanned {
                    value: Expression::NullLiteral,
                    span: token.span,
                })
            }
            
            Token::Literal(Literal::Undefined) => {
                self.advance();
                Ok(Spanned {
                    value: Expression::UndefinedLiteral,
                    span: token.span,
                })
            }

            Token::Identifier(name) => {
                self.advance();
                
                Ok(Spanned {
                    value: Expression::Identifier(name.clone()),
                    span: token.span
                })
            }

            Token::Punctuation(Punctuation::LeftParen) => {
                let tuple = self.parse_tuple()?;
                let span = tuple.span;

                Ok(Spanned {
                    value: Expression::Tuple(tuple),
                    span,
                })
            }

            Token::Template(_) => {
                let template = self.parse_template()?;
                let span = template.span;

                Ok(Spanned {
                    value: Expression::Template(template),
                    span
                })
            }

            _ => Err(ParserError::ExpectedExpression {
                found: token.clone(),
            }),
        }
    }

    /// Placeholder for LogicalNOT.
    fn parse_expresion_prefix(&mut self) {

    }

    fn parse_expression_postfix(
        &mut self, mut expression: Spanned<Expression>
    ) -> Result<Spanned<Expression>, ParserError> {
        loop {
            match self.current() {
                Some(token) if matches!(
                    token.value,
                    Token::Punctuation(Punctuation::Dot)
                ) => {
                    self.advance();

                    let member = self.parse_identifier()?;

                    let span = expression.span.until(member.span);

                    expression = Spanned {
                        value: Expression::MemberAccess {
                            object: Box::new(expression),
                            member
                        },
                        span,
                    };
                }
            
            Some(token) if matches!(
                token.value,
                Token::Punctuation(Punctuation::LeftParen)
            ) => {
                let arguments = self.parse_tuple()?;

                let span = expression.span.until(arguments.span);

                expression = Spanned {
                    value: Expression::Call {
                        callable: Box::new(expression),
                        arguments,
                    },
                    span,
                }
            }

            Some(token) if matches!(
                token.value,
                Token::Punctuation(Punctuation::LeftBracket)
            ) => {
                // parse index
            }

                _ => break,
            }
        }
        Ok(expression)
    }

    fn parse_expression_binary(
        &mut self,
        mut left: Spanned<Expression>,
        minimum_binding_power: u8,
    ) -> Result<Spanned<Expression>, ParserError> {
        loop {
            let Some(token) = self.current() else {
                break;
            };

            let Token::Operator(operator) = &token.value else {
                break;
            };

            let Some((left_binding_power, right_binding_power)) = operator.binding_power()
            else {
                break;
            };

            if left_binding_power < minimum_binding_power {
                break;
            }

            let operator = Spanned {
                value: operator.clone(),
                span: token.span,
            };

            self.advance();

            let mut right = self.parse_primary_expression()?;
            right = self.parse_expression_postfix(right)?;
            right = self.parse_expression_binary(right, right_binding_power)?;

            let span = left.span.until(right.span);

            let value = if operator.value.is_assignment() {
                Expression::Assignment {
                    target: Box::new(left),
                    operator,
                    value: Box::new(right),
                }
            } else {
                Expression::Binary {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                }
            };

            left = Spanned {
                value,
                span,
            };
        }

        Ok(left)
    }
}
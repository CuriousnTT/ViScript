use crate::ast::{
    statement::Statement,
    tuple_label::TupleLabelStyle,
    type_expression::TypeExpression,
};
use crate::parser::error::ParserError;
use crate::parser::parser::Parser;
use crate::source::span::Spanned;
use crate::token::{
    keyword::Keyword,
    operator::Operator,
    punctuation::Punctuation,
    token::Token,
};

impl<'a> Parser<'a> {
    pub(super) fn parse_statement(&mut self) -> Result<Spanned<Statement>, ParserError> {
        let current = match self.current() {
            Some(token) => token,
            None => return Err(ParserError::UnexpectedEof {
                span: self.cursor.current_span(),
            })
        };

        let statement = match current.value {
            Token::Keyword(Keyword::Export) => {
                return self.parse_export_statement()
            }
            Token::Keyword(Keyword::Function) => {
                return self.parse_function_declaration()
            },
            Token::Keyword(Keyword::If) => {
                return self.parse_if_statement()
            }
            Token::Keyword(Keyword::Let | Keyword::Const | Keyword::Rel) => {
                return self.parse_variable_declaration()
            },
            Token::Keyword(Keyword::Return) => {
                return self.parse_return_statement()
            },

            Token::Identifier(_) => {
                return self.parse_expression_statement()
            }
            _ => return Err(ParserError::ExpectedStatement { found: current.clone() })
        };
    }

    pub(super) fn parse_export_statement(&mut self) -> Result<Spanned<Statement>, ParserError> {
        let keyword = self.expect_keyword(Keyword::Export)?;

        let default = self.consume_keyword(Keyword::Default);

        let statement = self.parse_statement()?;

        let span = keyword.span.until(statement.span);

        Ok(Spanned {
            value: Statement::ExportDeclaration {
                keyword,
                default,
                declaration: Box::new(statement),
            },
            span
        })
    }

    pub(super) fn parse_expression_statement(&mut self) -> Result<Spanned<Statement>, ParserError> {
        let expression = self.parse_expression()?;

        let semicolon= self.expect_punctuation(Punctuation::Semicolon)?;

        let span = expression.span.until(semicolon.span);

        Ok(Spanned {
            value: Statement::Expression(expression),
            span,
        })
    }

    pub(super) fn parse_function_declaration(&mut self) -> Result<Spanned<Statement>, ParserError> {

        let async_ = self.consume_keyword(Keyword::Async);

        let keyword = self.expect_keyword(Keyword::Function)?;

        let identifier = self.parse_identifier()?;

        let colon = self.consume_punctuation(Punctuation::Colon);

        // Must be a named tuple or alias for labeled tuple.
        // Will be further resolved in semantic analysis.
        let parameters = self.parse_type_expression()?;

        // resolve (*not* validate) ambiguity for Tuple TypeExpressions
        // if colon, accept Type Alias identifiers, deferring to semantic analysis
        let parameters = parameters.map(
            |type_expression| match type_expression {
                TypeExpression::Tuple(tuple) => {
                    TypeExpression::Tuple(
                        tuple.map(|tuple_type| {
                            tuple_type.resolve_as(TupleLabelStyle::Labeled)
                        })
                    )
                }
                
                TypeExpression::Identifier(_) if colon.is_some() => type_expression,

                other => {
                    other // error case
                },
            }
        );

        let body = self.parse_block()?;

        let span = match &async_ {
            Some(async_keyword) => async_keyword.span.until(body.span),
            None => keyword.span.until(body.span),
        };

        Ok(Spanned {
            value: Statement::FunctionDeclaration {
                async_,
                keyword,
                identifier,
                parameters,
                body,
            },
            span
        })
    }

    pub(super) fn parse_if_statement(&mut self) -> Result<Spanned<Statement>, ParserError> {
        let if_keyword = self.expect_keyword(Keyword::If)?;

        let condition: Spanned<crate::ast::tuple::Tuple> = self.parse_tuple()?;

        let then = self.parse_block()?;

        let else_ = if self.consume_keyword(Keyword::Else).is_some() {
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };

        let end_span = if let Some(else_) = &else_ {
            else_.span
        } else {
            then.span
        };

        let span = if_keyword.span.until(end_span);

        Ok(Spanned {
            value: Statement::If {
                condition,
                then,
                else_
            },
            span
        })
    }

    pub(super) fn parse_return_statement(&mut self) -> Result<Spanned<Statement>, ParserError> {
        let keyword = self.expect_keyword(Keyword::Return)?;

        let value = if self.current().is_some_and(|token| {
            matches!(
                token.value,
                Token::Punctuation(Punctuation::Semicolon)
            )
        }) {
            None
        } else {
            Some(self.parse_expression()?)
        };

        let semicolon = self.expect_punctuation(Punctuation::Semicolon)?;
        let span = keyword.span.until(semicolon.span);

        Ok(Spanned {
            value: Statement::Return {
                keyword,
                value,
            },
            span,
        })
    }

    pub(super) fn parse_variable_declaration(&mut self) -> Result<Spanned<Statement>, ParserError> {
        let keyword = self.expect_one_of_keywords(&[
            Keyword::Let,
            Keyword::Const,
            Keyword::Rel,
        ])?;

        let identifier = self.parse_identifier()?;

        let value = if self.consume_operator(Operator::Assignment).is_some() {
            Some(self.parse_expression()?)
        } else {
            None
        };

        // NOTE: 'rel' and 'const' require an explicit and immediate assignment

        let semicolon = self.expect_punctuation(Punctuation::Semicolon)?;

        let span = keyword.span.until(semicolon.span);

        Ok(Spanned {
            value: Statement::VariableDeclaration {
                keyword,
                identifier,
                value,
            },
            span
        })
    }
}
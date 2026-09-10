use crate::ast::{
    statement_list::{
        Program,
        Block,
    },
    statement::Statement,
    expression::Expression,
    tuple::Tuple,
    tuple_position::TuplePosition,
};
use crate::parser::cursor::TokenCursor;
use crate::parser::error::ParserError;
use crate::source::span::Spanned;
use crate::token::{
    keyword::Keyword,
    literal::Literal,
    operator::Operator,
    punctuation::Punctuation,
    token::Token,
};

/// semantics:
/// consume -> move past optional syntax,
/// expect -> move past required syntax,
/// parse -> move past while extracting value,
pub struct Parser<'a> {
    cursor: TokenCursor<'a>,
    errors: Vec<ParserError>,
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
                    println!("{:?}", statement);
                    statements.push(statement);
                }

                Err(error) => {
                    self.errors.push(error);

                    // error recovery
                    panic!("TODO")
                }
            }
        }

        panic!("TODO")
    }

    fn parse_statement(&mut self) -> Result<Spanned<Statement>, ParserError> {
        let current = match self.current() {
            Some(token) => token,
            None => panic!("TODO")
        };

        match current.value {
            Token::Keyword(Keyword::Let) => {
                return self.parse_variable_declaration();
            },
            Token::Keyword(Keyword::Function) => {
                return self.parse_function_declaration();
            }
            Token::Identifier(_) => {
                return self.parse_expression_statement();
            }
            _ => panic!("TODO")
        }

    }

    fn parse_expression_statement(&mut self) -> Result<Spanned<Statement>, ParserError> {
        let expression = self.parse_expression()?;

        let semicolon= self.expect_punctuation(Punctuation::Semicolon)?;

        let span = expression.span.until(semicolon.span);

        Ok(Spanned {
            value: Statement::Expression(expression),
            span,
        })
    }

    fn parse_block(&mut self) -> Result<Block, ParserError> {
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
    fn parse_identifier(&mut self) -> Result<Spanned<String>, ParserError> {
        let Some(token) = self.current() else {
            panic!("Token stream ended abruptly");
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
            _ => Err(ParserError::ExpectedToken {
                expected: Token::Identifier(String::new()),
                found: token.clone(), 
            }),
        }
    }

    fn parse_expression(& mut self) -> Result<Spanned<Expression>, ParserError> {
        let mut expression = self.parse_primary_expression()?;

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

    fn parse_primary_expression(&mut self) -> Result<Spanned<Expression>, ParserError> {
        let Some(token) = self.current() else {
            panic!("Token stream ended abruptly");
        };

        match &token.value {
            Token::Literal(Literal::Numeric(value)) => {
                let literal = Spanned {
                    value: value.clone(),
                    span: token.span,
                };

                self.advance();

                Ok(Spanned {
                    value: Expression::NumericLiteral(literal),
                    span: token.span,
                })
            }

            Token::Identifier(name) => {
                let identifier = Spanned {
                    value: name.clone(),
                    span: token.span,
                };

                self.advance();
                
                Ok(Spanned {
                    value: Expression::Identifier(identifier),
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

            _ => Err(ParserError::ExpectedExpression {
                found: token.clone(),
            }),
        }
    }

    fn parse_variable_declaration(&mut self) -> Result<Spanned<Statement>, ParserError> {
        let keyword = self.expect_keyword(Keyword::Let)?;

        let identifier = self.parse_identifier()?;

        self.expect_operator(Operator::Assignment)?;

        let value = self.parse_expression()?;

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

    fn parse_function_declaration(&mut self) -> Result<Spanned<Statement>, ParserError> {

        let keyword = self.expect_keyword(Keyword::Function)?;
        let identifier = self.parse_identifier()?;

        // conceptually this is an empty tuple
        // long-term we will require an empty tuple or a named tuple type declaration here.
        self.expect_punctuation(Punctuation::LeftParen)?;
        self.expect_punctuation(Punctuation::RightParen)?;

        let body = self.parse_block()?;
        
        let span = keyword.span.until(body.span);

        Ok(Spanned {
            value: Statement::FunctionDeclaration {
                keyword,
                identifier,
                body,
            },
            span
        })
    }

    fn parse_tuple(&mut self) -> Result<Spanned<Tuple>, ParserError> {
        let opening = self.expect_punctuation(Punctuation::LeftParen)?;

        let mut positions: Vec<Spanned<TuplePosition>> = Vec::new();

        if self.current().is_some_and(|token| {
            matches!(
                token.value,
                Token::Punctuation(Punctuation::RightParen)
            )
        }) {
            let closing = self.expect_punctuation(Punctuation::RightParen)?;
            
            return Ok(Spanned {
                value: Tuple { positions },
                span: opening.span.until(closing.span),
            });
        }

        loop {
            positions.push(self.parse_tuple_position()?);

            if self.consume_punctuation(Punctuation::Comma).is_some() {
                continue;
            }

            let closing = self.expect_punctuation(Punctuation::RightParen)?;

            return Ok(Spanned {
                value: Tuple { positions },
                span: opening.span.until(closing.span),
            });
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

    // Consumes -> Optional consumption if present

    /// Generic consume helper
    fn consume_token<F, T>(
        &mut self, extract: F
    ) -> Option<Spanned<T>>
        where F: FnOnce(&Token) -> Option<T> {
        let token = self.current()?;

        let value = extract(&token.value)?;
        let span = token.span;

        self.advance();
        Some(Spanned { value, span })
    }

    fn consume_keyword(&mut self, keyword: Keyword) -> Option<Spanned<Keyword>> {
        self.consume_token(|token| {
            match token {
                Token::Keyword(k) if *k == keyword => Some(k.clone()),
                _ => None,
            }
        })
    }

    fn consume_operator(&mut self, operator: Operator) -> Option<Spanned<Operator>> {
        self.consume_token(|token| {
            match token {
                Token::Operator(o) if *o == operator => Some(o.clone()),
                _ => None,
            }
        })
    }

    fn consume_punctuation(&mut self, punctuation: Punctuation) -> Option<Spanned<Punctuation>> {
        self.consume_token(|token| {
            match token {
                Token::Punctuation(p) if *p == punctuation => Some(p.clone()),
                _ => None,
            }
        })
    }

    // Expect -> required
    fn expect_keyword(&mut self, keyword: Keyword) -> Result<Spanned<Keyword>, ParserError> {
        if let Some(token) = self.consume_keyword(keyword.clone()) {
            return Ok(token);
        }

        let found = self.current()
            .expect("Token stream ended abruptly").clone();

        Err(ParserError::ExpectedToken {
            expected: Token::Keyword(keyword),
            found,
        })
    }

    fn expect_operator(&mut self, operator: Operator) -> Result<Spanned<Operator>, ParserError> {
        if let Some(token) = self.consume_operator(operator.clone()) {
            return Ok(token);
        }

        let found = self.current()
            .expect("Token stream ended abruptly").clone();

        Err(ParserError::ExpectedToken {
            expected: Token::Operator(operator),
            found,
        })
    }

    fn expect_punctuation(&mut self, punctuation: Punctuation) -> Result<Spanned<Punctuation>, ParserError> {
        if let Some(token) = self.consume_punctuation(punctuation.clone()) {
            return Ok(token)
        }

        let found = self.current()
            .expect("Token stream ended abruptly").clone();

        Err(ParserError::ExpectedToken {
            expected: Token::Punctuation(punctuation),
            found
        })
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
    fn current_matches_tuple_label(&self) -> bool {
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

    // Cursor access
    fn current(&self) -> Option<&'a Spanned<Token>> {
        self.cursor.current()
    }

    fn peek(&self, offset: usize) -> Option<&'a Spanned<Token>> {
        self.cursor.peek(offset)
    }

    fn advance(&mut self) -> Option<&'a Spanned<Token>> {
        self.cursor.advance()
    }
}

/// Create a Parser and use it to parse a slice of tokens as an AST
pub fn parse(tokens: &[Spanned<Token>]) -> ParseResult {
    let mut parser = Parser::new(tokens);
    parser.parse_program()
}

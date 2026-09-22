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
    // Expect -> required

    fn expect_token<T, F, E>(
        &mut self, consume: F, expected: E
    ) -> Result<Spanned<T>, ParserError>
    where
        F: FnOnce(&mut Self) -> Option<Spanned<T>>,
        E: Into<Vec<Token>>,
    {
        if let Some(value) = consume(self) {
            return Ok(value);
        }

        let found = match self.current() {
            Some(token) => token.clone(),
            None => {
                return Err(ParserError::UnexpectedEof {
                    span: self.cursor.current_span(),
                });
            }
        };

        Err(ParserError::UnexpectedToken {
            expected: expected.into(),
            found
        })
    }

    pub(super) fn expect_keyword(
        &mut self, keyword: Keyword
    ) -> Result<Spanned<Keyword>, ParserError> {
        self.expect_token(
            |parser| parser.consume_keyword(keyword),
            vec![Token::Keyword(keyword)]
        )
    }

    pub(super) fn expect_one_of_keywords(
        &mut self, keywords: &[Keyword]
    ) -> Result<Spanned<Keyword>, ParserError> {
        self.expect_token(
            |parser| parser.consume_keyword_from_slice(keywords),
            keywords.iter().cloned().map(Token::Keyword).collect::<Vec<_>>(),
        )
    }

    pub(super) fn expect_operator(
        &mut self, operator: Operator
    ) -> Result<Spanned<Operator>, ParserError> {
        self.expect_token(
            |parser| parser.consume_operator(operator),
            vec![Token::Operator(operator)]
        )
    }

    pub(super) fn expect_one_of_operators(
        &mut self, operators: &[Operator]
    ) -> Result<Spanned<Operator>, ParserError> {
        self.expect_token(
            |parser| parser.consume_operator_from_slice(operators),
            operators.iter().cloned().map(Token::Operator).collect::<Vec<_>>(),
        )
    }

    pub(super) fn expect_punctuation(
        &mut self, punctuation: Punctuation
    ) -> Result<Spanned<Punctuation>, ParserError> {
        self.expect_token(
            |parser| parser.consume_punctuation(punctuation),
            vec![Token::Punctuation(punctuation)]
        )
    }

    pub(super) fn expect_one_of_punctuations(
        &mut self, punctuations: &[Punctuation]
    ) -> Result<Spanned<Punctuation>, ParserError> {
        self.expect_token(
            |parser| parser.consume_punctuation_from_slice(punctuations),
            punctuations.iter().cloned().map(Token::Punctuation).collect::<Vec<_>>(),
        )
    }
}
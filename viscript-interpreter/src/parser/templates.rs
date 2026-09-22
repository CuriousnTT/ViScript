use crate::ast::{
    template::Template,
    template_part::TemplatePart,
};
use crate::parser::error::ParserError;
use crate::parser::parser::Parser;
use crate::source::span::Spanned;
use crate::token::{
    template::Template as TemplateToken,
    token::Token,
};

impl<'a> Parser<'a> {
    pub(super) fn parse_template(&mut self) -> Result<Spanned<Template>, ParserError> {
        let start_span = self.cursor.current_span();

        let mut parts: Vec<TemplatePart> = Vec::new();

        match self.current() {
            Some(Spanned {
                value: Token::Template(TemplateToken::Complete(_)),
                span,
            }) => {
                parts.push(self.parse_template_text()?);

                return Ok(Spanned {
                    value: Template {
                        parts
                    },
                    span: *span,
                });
            }

            Some(Spanned {
                value: Token::Template(TemplateToken::Head(_)),
                ..
            }) => {
                parts.push(self.parse_template_text()?);
            }

            _ => {
                // not a template
                unreachable!("parse_template called when current token is not the start of a template")
            }
        };

        // loop Expression -> Middle/Tail, exit after Tail
        loop {
            let expression = self.parse_expression()?;
            let expression_span = expression.span;

            parts.push(TemplatePart::Expression(expression));

            match self.current() {
                Some(Spanned {
                    value: Token::Template(TemplateToken::Middle(_)),
                    ..
                }) => {
                    parts.push(self.parse_template_text()?);
                }

                Some(Spanned {
                    value: Token::Template(TemplateToken::Tail(_)),
                    span,
                }) => {
                    parts.push(self.parse_template_text()?);

                    return Ok(Spanned {
                        value: Template { parts },
                        span: start_span.until(*span),
                    });
                }

                _ => {
                    panic!("TODO");
                }
            }
        }
    }

    fn parse_template_text(
        &mut self,
    ) -> Result<TemplatePart, ParserError> {
        let Some(Spanned {
            value: Token::Template(template),
            span,
        }) = self.current()
        else {
            panic!("TODO");
        };
        
        let text = match template {
            TemplateToken::Complete(text)
            | TemplateToken::Head(text)
            | TemplateToken::Middle(text)
            | TemplateToken::Tail(text) => text,
        };

        self.advance();

        Ok(TemplatePart::Text(Spanned {
            value: text.clone(),
            span: *span,
        }))
    }
}
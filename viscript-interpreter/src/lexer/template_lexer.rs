use crate::lexer::{
    error::LexerError,
    template::Template,
    token::Token,
    tokenizer::{
        Lexer,
        tokenize_range,
    }
};
use crate::source::{
    span::{
        Span,
        Spanned,
    },
};

impl <'a> Lexer <'a> {
    pub(super) fn scan_template(&mut self) {
        let mut template_tokens: Vec<Spanned<Token>> = Vec::new();
        let mut template_errors: Vec<LexerError> = Vec::new();
        // wrappers are implied by Template Token type, so

        let start = self.pos; // only collected for errors
        self.advance(); // skip `
        let mut template_start = self.pos;
        
        let mut escaped = false;
        
        loop {
            let Some(ch) = self.peek() else {
                self.push_template_piece(
                    &mut template_tokens,
                    template_start,
                    |text| {
                        Template::Tail(text)
                    }
                );

                template_errors.push(
                    LexerError::UnterminatedTemplate {
                        span: Span::new(start, start + 1),
                    }
                );
                break;
            };

            if escaped {
                escaped = false;
                self.advance();
                continue;
            }

            match ch {
                '\\' => escaped = true,

                '$' => {
                    if self.peek_next() == Some('{') {
                        let part_end = self.pos; // before ${
                        let is_first = template_tokens.is_empty();
                        self.push_template_piece(
                            &mut template_tokens,
                            template_start,
                            |text| {
                                if is_first {
                                    Template::Head(text)
                                } else {
                                    Template::Middle(text)
                                }
                            }
                        );

                        self.advance(); self.advance(); // the ${
                        let interpolation_start = self.pos;
                        let interpolation_end = match self.find_expression_end() {
                            Some(end) => end,
                            None => {
                                // unterminated interpolation
                                self.push_template_piece(
                                    &mut template_tokens,
                                    interpolation_start,
                                    |text| {
                                        Template::Tail(text)
                                    }
                                );

                                template_errors.push(LexerError::UnterminatedInterpolation {
                                    span: Span::new(part_end, interpolation_start)
                                });
                                break;
                            }
                        };

                        let inner_result = tokenize_range(
                            self.source,
                            self.vsx_enabled,
                            interpolation_start,
                            interpolation_end
                        );
                        template_errors.extend(inner_result.errors);
                        template_tokens.extend(inner_result.tokens);
                        self.pos = interpolation_end + 1; // skip to closing }
                        template_start = self.pos;
                        continue;
                    }
                }
                '`' => {
                    // make final token (Complete or Tail)
                    let is_first = template_tokens.is_empty();
                    self.push_template_piece(
                        &mut template_tokens,
                        template_start,
                        |text| {
                            if is_first {
                                Template::Complete(text)
                            } else {
                                Template::Tail(text)
                            }
                        }
                    );
                    self.advance(); // closing `
                    break;
                }
                _ => {}
            }
            self.advance();
        }
        // Validate before feeding collected tokens/errors to self.tokens & self.errors
        // Skip validation for now to see how it behaves
        self.tokens.extend(template_tokens);
        self.errors.extend(template_errors);
    }

    /// Creates and pushes a token of the specified kind, ending on the current position.
    fn push_template_piece(
        &self,
        tokens: &mut Vec<Spanned<Token>>,
        template_start: usize,
        build: impl FnOnce(String) -> Template,
    ) {
        let end = self.pos;

        let text = self.source[template_start..end].to_string();

        tokens.push(Spanned {
            value: Token::Template(build(text)),
            span: Span::new(template_start, end),
        });
    }
}
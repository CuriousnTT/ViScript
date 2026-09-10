use std::str::FromStr;

use crate::lexer::{
    error::LexerError,
    regex::{
        VSX_IDENTIFIER_REGEX,
        VSX_IN_TAG_REGEX
    },

    tokenizer::{
        Lexer,
        tokenize_range,
    },
};
use crate::source::span::{
    Span,
    Spanned
};
use crate::token::{
    operator::Operator,
    punctuation::Punctuation,
    token::{
        Token,
        TokenKind,
    },
    vsx::{
        Tag,
        VSX,
    },
};

impl <'a> Lexer <'a> {
    pub(super) fn scan_vsx(&mut self) {
        let mut vsx_tokens: Vec<Spanned<Token>> = Vec::new();
        let mut vsx_errors: Vec<LexerError> = Vec::new();
        let start = self.pos;
        let mut token_start = start;
        let mut vsx_element_depth = 0;
        let mut in_tag = false;

        loop {
            let Some(ch) = self.peek() else {
                if vsx_element_depth == 0
                    && in_tag == false
                    && !vsx_tokens.is_empty() {
                    break; // Reached end of file, but we've also left VSX
                }
                if !in_tag {
                        self.push_vsx_token(
                        &mut vsx_tokens,
                        token_start,
                        |text| {
                            VSX::Text(text)
                        }
                    );
                }
                vsx_errors.push(
                    LexerError::UnterminatedVSXElement {
                        span: Span::new(start, start + 1)
                    }
                );
                break;
            };

            if in_tag {
                // scan one thing inside an element tag
                match ch {
                    // tag-enders
                    '/' if self.peek_next() == Some('>') => { // whitespace is NOT accepted here
                        vsx_element_depth -= 1; // ends tag and closes it -> decrement depth
                        self.advance(); // consume /
                        self.advance(); // consume >
                        self.push_vsx_token(
                            &mut vsx_tokens,
                            token_start,
                            |_text| {
                                VSX::Tag(Tag::CloseEnd)
                            }
                        );
                        token_start = self.pos;
                        in_tag = false;
                        continue;
                    }
                    '>' => {
                        // ends tag but does not close it
                        // -> no decrement on element depth
                        self.advance(); // >
                        self.push_vsx_token(
                            &mut vsx_tokens,
                            token_start,
                            |_text| {
                                VSX::Tag(Tag::OpenEnd)
                            }
                        );
                        token_start = self.pos;
                        in_tag = false;
                        continue;
                    }
                    // tag contents
                    '<' if self.can_start_vsx_type_arguments(&vsx_tokens) => {
                        // needs nested lexing,
                        // including start of type argument
                        let type_start = self.pos;
                        self.advance(); // consume the <
                        let type_end = match self.find_type_argument_end() {
                            Some(end) => end + 1, // as it is not inclusive
                            None => {
                                vsx_errors.push(
                                    LexerError::UnterminatedTypeArgument {
                                        span: Span::new(
                                            type_start, type_start + '>'.len_utf8()
                                        )
                                    }
                                );
                                // no token
                                break;
                            }
                        };
                        let inner_result = tokenize_range(
                            self.source,
                            false,
                            type_start,
                            type_end
                        );
                        vsx_tokens.extend(inner_result.tokens);
                        vsx_errors.extend(inner_result.errors);
                        self.pos = type_end; // skip to closing >
                        token_start = self.pos;
                        continue;
                    }
                    '{' => {
                        if !self.scan_vsx_expression(
                            &mut vsx_tokens,
                            &mut vsx_errors
                        ) {
                            break;
                        }

                        token_start = self.pos;
                        continue;
                    }
                    _ => {
                        if !self.scan_vsx_tag_token(
                            &mut vsx_tokens,
                            &mut vsx_errors
                        ) {
                            break;
                        }
                        token_start = self.pos;
                        continue;
                    }
                }
            } else {
                if vsx_element_depth == 0 && !vsx_tokens.is_empty() {
                    // we've left VSX
                    break;
                }

                // scan one thing inside element contents
                match ch {
                    '<' => {
                        in_tag = true;
                        let is_closing = self.peek_next() == Some('/'); // VSX doesn't tolerate whitespace here.

                        let part_end = self.pos; // before {
                        if !vsx_tokens.is_empty() {
                            self.push_vsx_token(
                                &mut vsx_tokens,
                                token_start,
                                |text| {
                                    VSX::Text(text)
                                }
                            );
                        }

                        if is_closing {
                            vsx_element_depth -= 1; // closing tag -> decrement
                            self.advance(); // consume <
                            self.advance(); // consume /

                            self.push_vsx_token(
                                &mut vsx_tokens,
                                part_end,
                                |_text| {
                                    VSX::Tag(Tag::CloseStart)
                                }
                            );
                        } else {
                            vsx_element_depth += 1; // entering a tag
                            self.advance(); // <
                            self.push_vsx_token(
                                &mut vsx_tokens,
                                part_end,
                                |_text| {
                                    VSX::Tag(Tag::OpenStart)
                                }
                            );
                        }
                        token_start = self.pos;
                        continue;
                        
                    }
                    '{' => {
                        // push Text
                        self.push_vsx_token(
                            &mut vsx_tokens,
                            token_start,
                            |text| {
                                VSX::Text(text)
                            }
                        );
                        // lex expression
                        if !self.scan_vsx_expression(
                            &mut vsx_tokens,
                            &mut vsx_errors,
                        ) {
                            break;
                        }
                        token_start = self.pos;
                    }
                    _ => {
                        self.advance(); // consume arbitrary character
                    }
                }
            }
        }

        self.tokens.extend(vsx_tokens);
        self.errors.extend(vsx_errors);
        self.expr_allowed = false;
    }

    fn can_start_vsx_type_arguments(
        &self,
        vsx_tokens: &Vec<Spanned<Token>>
    ) -> bool {
        match(
            vsx_tokens.get(vsx_tokens.len().wrapping_sub(2)),
            vsx_tokens.last(),
        ) {
            (
            Some(Spanned {
                value: Token::VSX(VSX::Tag(Tag::OpenStart)),
                ..
            }),
            Some(Spanned {
                value: Token::VSX(VSX::Identifier(_)),
                ..
            }),
        ) => true,
        _ => false,
        }
        
    }

    fn scan_vsx_expression(
        &mut self,
        vsx_tokens: &mut Vec<Spanned<Token>>,
        vsx_errors: &mut Vec<LexerError>,
    ) -> bool {
        let expression_opener = self.pos;
        self.advance(); // consume {

        let expression_start = self.pos;
        let opener_span = Span::new(
            expression_opener,
            expression_start
        );
        let opener_token = Spanned{
            value:Token::VSX(VSX::ExpressionStart),
            span: opener_span
        };
        vsx_tokens.push(opener_token);

        let expression_end = match self.find_expression_end() {
            Some(end) => end,
            None => {
                vsx_errors.push(LexerError::UnterminatedVSXExpression {
                    span:  opener_span,
                });
                return false;
            }
        };

        let inner_result = tokenize_range(
            self.source,
            self.vsx_enabled,
            expression_start,
            expression_end
        );
    
        self.pos = expression_end;
        self.advance(); // more idiomatic than + 1 for skipping }

        let closer_token = Spanned{
            value:Token::VSX(VSX::ExpressionEnd),
            span: Span::new(
                expression_end,
                self.pos,
            )
        };

        vsx_tokens.extend(inner_result.tokens);
        vsx_errors.extend(inner_result.errors);
        vsx_tokens.push(closer_token);

        true
    }

    fn scan_vsx_tag_token(
        &mut self,
        vsx_tokens: &mut Vec<Spanned<Token>>,
        vsx_errors: &mut Vec<LexerError>,
    ) -> bool {
        let remaining = &self.source[self.pos..];
        for (regex, kind) in VSX_IN_TAG_REGEX.iter() {
            if let Some(mat) = regex.find(remaining) {
                if mat.start() == 0 {
                    let length = mat.end();
                    let start = self.pos;
                    let end = self.pos + length;
                    let text = mat.as_str();
                    let span = Span { start, end };
                    let token = match &kind {
                        TokenKind::Skip => {
                            self.pos += length;
                            return true;
                        }

                        TokenKind::Identifier => {
                            // no keywords to disentangle from Identifiers here!
                            Token::VSX(VSX::Identifier(text.to_string()))
                        }

                        TokenKind::String => {
                            Token::VSX(VSX::String(text.to_string()))
                        }

                        TokenKind::Operator => match Operator::from_str(text) {
                            Ok(op) => Token::Operator(op),
                            Err(_) => {
                                let ch = text.chars().next().unwrap_or('?');
                                vsx_errors.push(
                                    LexerError::UnexpectedCharacter {
                                        char: ch,
                                        span
                                    }
                                );
                                self.pos += ch.len_utf8();
                                return false;
                            }
                        }
                        TokenKind::Punctuation => {
                            match Punctuation::from_str(text) {
                                Some(p) => Token::Punctuation(p),
                                None => {
                                    let ch = text.chars().next().unwrap_or('\0');
                                    vsx_errors.push(
                                        LexerError::UnexpectedCharacter {
                                            char: ch,
                                            span,
                                        }
                                    );
                                    self.pos += ch.len_utf8();
                                    return false;
                                }
                            }
                        }
                        _ => {
                            let ch = text.chars().next().unwrap();
                            vsx_errors.push(LexerError::UnexpectedCharacter {
                                char: ch,
                                span
                            });
                            self.pos += ch.len_utf8();
                            return false;
                        }
                    };

                    self.pos += length;
                    vsx_tokens.push(Spanned {
                        value: token,
                        span
                    });
                    return true;
                }
            }
        }

        // no match
        let ch = self.peek().unwrap();

        vsx_errors.push(
            LexerError::UnexpectedCharacter {
                char: ch,
                span: Span {
                    start: self.pos,
                    end: self.pos + ch.len_utf8(),
                } }
        );
        self.advance(); // consume unexpected char
        false
    }

    /// Returns true if VSX scanning-conditions are met
    /// Mutates Lexer state during lookahead
    /// Reverts to pre-mutated state before returning
    pub(super) fn should_scan_vsx(&mut self) -> bool {
        if !self.expr_allowed {
            return false;
        }

        if let Some(last) = self.tokens.last() {
            if last.value.is_less_than_family() {
                return false;
            }
        }

        let start = self.pos;
        let coherent_as_vsx = self.inspect_as_vsx();
        self.pos = start;
        return coherent_as_vsx;
    }

    /// returns true if the sequence starting at self.pos
    /// is structurally valid as a VSX opening or self-closing element tag.
    /// 
    /// WARNING: Mutates position during inspection;
    /// Record self.pos before calling unless you intend to skip
    /// Then restore self.pos after calling.
    fn inspect_as_vsx(&mut self) -> bool {
        if self.peek() != Some('<') {
            return false; // we're not even looking at VSX
        }

        // special rules for the start of a VSX tag:
        // 1. If the next non-whitespace is > it is a fragment tag -> definitively VSX
        let next = match self.peek_next_non_whitespace(&['\n']) { // newline is not accepted here
            Some(next) if !next.is_whitespace() => next,
            Some(_) => return false, // extra check since newline is NOT accepted here.
            None => return false
        };
        if next == '>' {
            return true;
        }

        // 2. Non-fragment starts with a valid VSX identifier with possible namespace access
        let next_pos = match self.next_instance_of_char(next) {
            Some(pos) => pos,
            None => return false
        };
        
        // skip any whitespace between self.pos and next_pos
        self.pos += next_pos;
    
        if !self.inspect_vsx_tag_name() {
            return false;
        };

        // 4. Consistent rules afterwards:
        self.inspect_vsx_tag_attributes()
    }

    /// Reads first VSX-identifier until first whitespace or tag closer.
    /// Thus it covers any namespace accesses +
    /// chained identifiers +
    /// optional type argument list just before whitespace
    /// 
    /// Returns true if the structure is valid in a VSX opening tag
    /// 
    /// NOTE: Advances Lexer position; Use with care.
    fn inspect_vsx_tag_name(&mut self) -> bool {
        // first step of possible identifier/namespace chaining
        if !self.consume_vsx_identifier() {
            return false;
        };

        loop {
            match self.peek() {
                Some('.') => { // nested namespace access handler: Dot must be followed by identifier
                    self.advance();
                    if !self.consume_vsx_identifier() {
                        return false;
                    }
                }

                Some('<') => {
                    // Type argument starts
                    self.advance(); // pass the opener <
                    let type_argument_end = match self.find_type_argument_end() {
                        Some(end) => end + 1, // as it is not inclusive
                        None => return false,
                    };

                    self.pos = type_argument_end; // update position after closer >
                    return true;
                }

                Some(c) if c.is_whitespace() => {
                    return true;
                }

                Some('>') => {
                    return true;
                }
                
                Some('/') if self.peek_next() == Some('>') => {
                    return true;
                }

                Some(_) => {
                    return false;
                }

                None => {
                    return false;
                }
            }
        }
    }

    fn inspect_vsx_tag_attributes(&mut self) -> bool {
        let mut attribute_expected = false;

        loop {
            let c = match self.peek() {
                Some(c) => c,
                None => return false, // Hitting EOF before tag closure -> not VSX
            };

            match c {
                '>' => return !attribute_expected, // looks like VSX opening tag but might be broken -> a job for validation

                // Comments are legal
                '/' if self.peek_next() == Some('/') => {
                    self.skip_line_comment();
                }
                '/' if self.peek_next() == Some('*') => {
                    self.skip_block_comment();
                }

                // Tag may be self-closing and/or broken. If it is -> definitively VSX
                '/' => {
                    if attribute_expected {
                        return false; // wrong time for /
                    }

                    self.advance(); // pass /, look for closing angle

                    // following best practices we don't accept whitespace before closing angle
                    return self.peek() == Some('>');
                }
            
                // trivia
                c if c.is_whitespace() => {
                    self.advance();
                }

                // Strings are valid if the preceding non-whitespace was an assignment ('=')
                '"' | '\'' if attribute_expected => {
                    self.skip_string();
                    attribute_expected = false;
                }

                // JavaScript expressions/props-inheritance
                '{' if attribute_expected => {
                    self.advance(); // pass the opener {
                    let attribute_end = match self.find_expression_end() {
                        Some(end) => end,
                        None => return false,
                    };

                    self.pos = attribute_end + 1; // update position after closer }
                    attribute_expected = false; // found attribute
                }

                // Standalone {...} attribute/spread
                '{' => {
                    self.advance(); // pass the opener {
                    let attribute_end = match self.find_expression_end() {
                        Some(end) => end,
                        None => return false,
                    };

                    self.pos = attribute_end + 1; // update position after closer }
                }

                // separators that are only legal in first identifier -> not VSX
                '.' | '<' => {
                    return false;
                }

                // '=' sets up a possible string or JavaScript expression
                '=' => {
                    if attribute_expected {
                        return false; // not allowed consecutively
                    }
                    attribute_expected = true;
                    self.advance();
                }
                
                _ => {
                    if attribute_expected {
                        return false;
                    }

                    if !self.consume_vsx_identifier() {
                        return false;
                    }
                }
            }
        }
    }

    /// Advances past vsx_identifier at current position if one exists
    /// returns true if it did, false if not.
    fn consume_vsx_identifier(&mut self) -> bool {
        match self.vsx_identifier_at(self.pos) {
            Some(id) => {
                self.pos += id.len();
                true
            }
            None => false,
        }
    }

    fn vsx_identifier_at(&self, pos: usize) -> Option<&str> {
        let rest = &self.source[pos..];

        let m = VSX_IDENTIFIER_REGEX.find(rest)?;

        if m.start() != 0 {
            return None;
        }

        let text = &rest[..m.end()];

        Some(text)
    }

    fn push_vsx_token(
        &self,
        tokens: &mut Vec<Spanned<Token>>,
        token_start: usize,
        build: impl FnOnce(String) -> VSX
    ) {
        let end = self.pos;

        let text = self.source[token_start..end].to_string();

        tokens.push(Spanned {
            value: Token::VSX(build(text)),
            span: Span::new(token_start, end),
        });
    }
}
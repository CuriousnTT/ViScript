use crate::lexer::tokenizer::Lexer;

impl <'a> Lexer <'a> {
    /// Iterates until angle_depth is 0
    /// or it runs out of source code.
    /// or the depth by angle/paren/bracket/brace becomes negative (aka impossible)
    /// 
    /// Advances until it finds the closing angle
    /// Assumes opener was already consumed
    /// NOTE: Returned usize is not inclusive of the closing angle.
    /// 
    /// Intended use: Identify stopping point for nested lexers before they run
    pub(super) fn find_type_argument_end(&mut self) -> Option<usize> {
        let mut angle_depth = 1;
        let mut paren_depth = 0;
        let mut bracket_depth = 0;
        let mut brace_depth = 0;

        while !self.is_eof() {
            if angle_depth < 0
                || paren_depth < 0
                || bracket_depth < 0
                || brace_depth < 0 {
                return None;
            } 
            match self.peek()? {
                '<' if paren_depth == 0
                    && bracket_depth == 0
                    && brace_depth == 0 => {
                    angle_depth += 1
                }

                '>' if paren_depth == 0
                    && bracket_depth == 0
                    && brace_depth == 0 => {
                        angle_depth -= 1;
                        if angle_depth == 0 {
                            return Some(self.pos);
                        }
                }
                '(' => paren_depth += 1,
                ')' => paren_depth -= 1,

                '[' => bracket_depth += 1,
                ']' => bracket_depth -= 1,

                '{' => brace_depth += 1,
                '}' => brace_depth -= 1,
                
                '\'' | '"' | '`' => {
                    if !self.skip_string() {
                        self.advance();
                    }
                    continue;
                }

                '/' => {
                    if self.peek_n(1) == Some('/') {
                        self.skip_line_comment();
                        continue;
                    }

                    if self.peek_n(1) == Some('*') {
                        if !self.skip_block_comment() {
                            self.advance();
                            self.advance();
                        }
                        continue;
                    }
                }

                _ => {}
            }

            self.advance();
        }

        None
    }

    /// Iterates until brace_depth is zero
    /// or it runs out of source code.
    /// Advances until it finds the closing brace
    /// Assumes opening brace was already consumed
    /// ignores braces in comments/strings/template literals/regex (best effort)
    /// NOTE: Returned usize is not inclusive of the closing brace.
    /// 
    /// Intended use: Identify stopping point for nested lexers before they run
    pub(super) fn find_expression_end(&mut self) -> Option<usize> {
        let mut brace_depth = 1; // Opening brace should already be consumed

        while !self.is_eof() {
            match self.peek() {
                // closing/opening braces
                Some('}') => {
                    brace_depth -= 1;
                    if brace_depth == 0 {
                        let end_pos = self.pos;
                        return Some(end_pos);
                    }
                }
                Some('{') => {
                    brace_depth += 1;
                }

                // various constructs can contain braces that don't count

                // escape -> skips the next character
                Some('\\') => {
                    // Skip escaped characters
                    self.advance();
                    if !self.is_eof() {
                        self.advance();
                        continue;
                    }
                }
                // String/template literals
                Some('\'') | Some('"') | Some('`') => {
                    let string_start = self.pos;
                    if !self.skip_string() {
                        self.pos = string_start;
                        self.advance();
                    }
                    continue;
                }
                // comments
                Some('/') => {
                    match self.peek_n(1) {
                        // single-line comment
                        Some('/') => {
                            self.skip_line_comment();
                            continue;
                        }
                        // block comment: /* ... */
                        Some('*') => {
                            let comment_start = self.pos;
                            if !self.skip_block_comment() {
                                self.pos = comment_start;
                                self.advance();
                                self.advance();
                            }
                            continue;
                        }
                        // regex candidate
                        _ if self.looks_like_regex_start() => {
                            let regex_start = self.pos;
                            if !self.skip_regex_literal() {// defined in regex_lexer.rs
                                self.pos = regex_start;
                                self.advance();
                            }
                            continue;
                        }
                        
                        _ => {}
                    }
                }
                _ => {}
            }
            self.advance();
        }
        None
    }

    /// including template literals
    pub(super) fn skip_string(&mut self) -> bool {
        let quote = self.advance().unwrap();
        loop {
            if self.is_eof() {
                return false;
            }
            if self.peek() == Some('\\') {
                self.advance();
                self.advance();
                continue;
            }
            if self.peek() == Some(quote) {
                self.advance();
                break;
            }
            self.advance();
        }
        true 
    }

    /// Skips comment from /* to */
    pub(super) fn skip_block_comment(&mut self) -> bool {
        self.advance(); // /
        self.advance(); // *
        loop {
            if self.is_eof() {
                return false
            }
            if self.peek() == Some('*') && self.peek_n(1) == Some('/') {
                self.advance(); // *
                self.advance(); // /
                break;
            }
            self.advance();
        }
        true
    }

    // Skips rest of line after //
    pub(super) fn skip_line_comment(&mut self) -> bool {
        self.advance();
        self.advance();
        while let Some(c) = self.peek() {
            if c == '\n' || c == '\r' || c == '\u{2028}' || c == '\u{2029}' {
                break;
            }
            self.advance();
        }
        return true
    }

    /// Conservative heuristic
    fn looks_like_regex_start(&self) -> bool {
        match self.peek_n(1) {
            Some('/') | Some('*') => false, // comments already handled
            Some(c) if c.is_ascii_whitespace() => false, // rare at start of regex
            Some(c) if c.is_ascii_digit() => false, // likewise
            _ => true,
        }
    }
}
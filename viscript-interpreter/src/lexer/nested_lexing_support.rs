use crate::lexer::tokenizer::Lexer;

impl <'a> Lexer <'a> {
    /// Iterates until brace_depth is zero
    /// or it runs out of source code.
    /// Advances until it finds the closing brace
    /// Assumes opening brace was already consumed
    /// ignores braces in comments/strings/template literals/regex (best effort)
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
                    let quote = self.advance().unwrap();
                    while !self.is_eof() {
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
                    continue;
                }
                // comments
                Some('/') => {
                    match self.peek_n(1) {
                        // single-line comment
                        Some('/') => {
                            self.advance();
                            self.advance();
                            while let Some(c) = self.peek() {
                                if c == '\n' || c == '\r' || c == '\u{2028}' || c == '\u{2029}' {
                                    break;
                                }
                                self.advance();
                            }
                            continue;
                        }
                        // block comment: /* ... */
                        Some('*') => {
                            self.advance(); // /
                            self.advance(); // *
                            loop {
                                if self.is_eof() {
                                    break; // stop for unterminated
                                }
                                if self.peek() == Some('*') && self.peek_n(1) == Some('/') {
                                    self.advance(); // *
                                    self.advance(); // /
                                    break;
                                }
                                self.advance();
                            }
                            continue;
                        }
                        // regex candidate
                        _ if self.looks_like_regex_start() => {
                            self.skip_regex_literal();
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
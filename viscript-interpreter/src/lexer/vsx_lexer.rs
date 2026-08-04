use std::str::FromStr;
use crate::lexer::{
    error::LexerError,
    keyword::Keyword,
    regex::IDENTIFIER_REGEX,
    token::Token,
    tokenizer::Lexer,
};

impl <'a> Lexer <'a> {
    pub(super) fn scan_vsx(&mut self) {

    }

    pub(super) fn should_scan_vsx(&self) -> bool {
        if !self.expr_allowed {
            return false;
        }

        if let Some(last) = self.tokens.last() {
            if last.value.is_less_than_family() {
                return false;
            }
        }

        match self.peek_next_non_whitespace() {
            Some('/' | '>') => true, // closing tag or fragment
            Some(c) => {
                if let Some(pos) = self.next_instance_of_char(c) {
                    self.identifier_at(pos).is_some()
                } else {
                    false
                }

            }, // <SomeTag (NB! also matches types)
            _ => false,
        }
    }

    fn identifier_at(&self, pos: usize) -> Option<&str> {
        let rest = &self.source[pos..];

        let m = IDENTIFIER_REGEX.find(rest)?;

        if m.start() != 0 {
            return None;
        }

        let text = &rest[..m.end()];

        if Keyword::from_str(text).is_ok() {
            None
        } else {
            Some(text)
        }
    }
}
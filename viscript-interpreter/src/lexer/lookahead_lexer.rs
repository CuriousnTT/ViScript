use crate::lexer::tokenizer::Lexer;

/// Lookahead methods may mutate Lexer,
/// But they should always leave it as they found it.
impl<'a> Lexer<'a> {
    /// Peek at the next character in the source string.
    /// Equivalent to peek_n(n) when n = 0
    pub(super) fn peek(&self) -> Option<char> {
        self.source[self.pos..].chars().next()
    }

    /// Peek 1 step ahead in the source string.
    /// Equivalent to the method peek_n(n) when n = 1
    pub(super) fn peek_next(&self) -> Option<char> {
        self.source[self.pos..].chars().nth(1)
    }

    /// Peek n characters ahead in the source string.
    /// For n = 0, equivalent to the peek method.
    /// For n = 1, equivalent to the peek_next method.
    pub(super) fn peek_n(&self, n: usize) -> Option<char> {
        self.source[self.pos..].chars().nth(n)
    }

    /// Peek at the next character that is not whitespace
    /// Optionally stop early for any whitespace character listed in significant
    pub(super) fn peek_next_non_whitespace(&self, significant: &[char]) -> Option<char> {
        for c in self.source[self.pos..].chars().skip(1) {
            if c.is_whitespace() {
                // if c in significant, treat as the answer
                if significant.contains(&c) {
                    return Some(c);
                }
                continue;
            }

            return Some(c);
        }
        None
    }

    /// Returns the byte offset from self.pos to next case of 'c'
    /// Returns 'None' if it is not found
    pub(super) fn next_instance_of_char(&self, c: char) -> Option<usize> {
        self.source[self.pos..]
            .char_indices()
            .find(
            |(_, ch) | *ch == c)
            .map(|(offset, _) | offset)
    }
}
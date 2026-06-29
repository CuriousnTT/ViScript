#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn line_column(&self, file: &SourceFile) -> (usize, usize) {
        let pos = self.start;
        let line = match file.line_starts.binary_search(&pos) {
            Ok(line) => line,
            Err(next_line) => next_line - 1,
        };

        let line_start = file.line_starts[line];
        let column = pos - line_start;

        // human-friendly indexing
        (line + 1, column + 1)
    }
}

pub fn span_from_match(pos: usize, mat: &regex::Match) -> Span {
    Span { start: pos, end: pos + mat.end() }
}


#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub value: T,
    pub span: Span,
}

impl<T: std::fmt::Debug> std::fmt::Display for Spanned<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

pub struct SourceFile {
    pub source: String,
    pub line_starts: Vec<usize>, // byte offsets of each line
}

impl SourceFile {
    pub fn new(source: String) -> Self {
        let mut line_starts = vec![0];

        for (i, ch) in source.char_indices() {
            if ch == '\n' {
                line_starts.push(i + 1);
            }
        }

        Self { source, line_starts }
    }

    /// returns the line number (0-based) and column (0-based) for a byte position
    pub fn line_column(&self, pos: usize) -> (usize, usize) {
        let line = match self.line_starts.binary_search(&pos) {
            Ok(line) => line,
            Err(next_line) => next_line.saturating_sub(1),
        };
        let line_start = self.line_starts[line];
        let column = pos - line_start;
        (line, column)
    }

    /// Get surrounding context for an error
    pub fn get_error_context(&self, span: Span, context_lines: usize) -> String {
        if self.source.is_empty() {
            return String::new();
        }

        let (err_start_line, err_start_col) = self.line_column(span.start);
        let (err_end_line, err_end_col) = self.line_column(span.end.min(self.source.len()));

        let start_line = err_start_line.saturating_sub(context_lines);
        let end_line = (err_end_line + context_lines + 1).min(self.line_starts.len() -1);

        let mut output = String::new();

        for line_num in start_line..=end_line {
            let line_start = self.line_starts[line_num];
            let line_end = if line_num + 1 < self.line_starts.len() {
                self.line_starts[line_num + 1] - 1 // exclude newline
            } else {
                self.source.len()
            };

            let line_text = &self.source[line_start..line_end];

            let line_header = if line_num == err_start_line {
                format!("{:>4} | ", line_num + 1)
            } else {
                format!("    |  ")
            };

            output.push_str(&format!("{}{}\n", line_header, line_text));

            // Multiline underline for error span
            if line_num >= err_start_line && line_num <= err_end_line {
                let mut underline = String::new();

                let left = if line_num == err_start_line { err_start_col } else { 0 };
                let right = if line_num == err_end_line { err_end_col } else { line_text.len() };

                if left < right {
                    underline.push_str(&" ".repeat(left));
                    underline.push_str(&"^".repeat(right - left));
                }

                if !underline.is_empty() {
                    output.push_str(&format!(
                        "{:>width$}{}\n", "", underline, width = line_header.len()
                    ));
                }
            }
        }

        output
    }
}

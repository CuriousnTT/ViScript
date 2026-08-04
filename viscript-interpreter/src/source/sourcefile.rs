use std::fs;
use std::path::PathBuf;

use crate::source::span::Span;

pub struct SourceFile {
    pub file_path: PathBuf,
    pub source: String,
    pub line_starts: Vec<usize>, // byte offsets of each line
}

impl SourceFile {
    pub fn new(filename: &str) -> Result<Self, std::io::Error> {
        //Read and adapt test file to avoid errors
        let source = fs::read_to_string(filename)?
            .replace("\r\n", "\n");

        let mut line_starts = vec![0];

        for (i, ch) in source.char_indices() {
            if ch == '\n' {
                line_starts.push(i + 1);
            }
        }

        Ok(
            Self {
                file_path: PathBuf::from(filename),
                source,
                line_starts
            }
        )
    }

    /// returns the line number (0-based) and column (0-based) for a byte position
    pub fn line_column(&self, pos: usize) -> (usize, usize) {
        if pos > self.source.len() {
            return (self.line_starts.len().saturating_sub(1), 0);
        }
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

            let line_header = format!("{:>4} |  ", line_num + 1);


            output.push_str(&format!("{}{}\n", line_header, line_text));

            // Multiline underline for error span
            if line_num >= err_start_line && line_num <= err_end_line {
                let mut underline = String::new();

                let left: usize = if line_num == err_start_line { err_start_col } else { 0 };
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

    /// returns true for file extensions .jsx, .tsx & .vsx.
    pub fn is_vsx(&self) -> bool {
        matches!(
            self.file_path.extension().and_then(|s| s.to_str()),
            Some("jsx" | "tsx" | "vsx")
        )
    }
}

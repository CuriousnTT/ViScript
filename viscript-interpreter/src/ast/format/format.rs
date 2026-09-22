use std::fmt::Write;

pub struct Formatter {
    output: String,
    indent: usize,
}

impl Formatter {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            indent: 0,
        }
    }

    pub fn finish(self) ->  String {
        self.output
    }

    pub fn line(&mut self, text: impl std::fmt::Display) {
        for _ in 0..self.indent {
            self.output.push_str("  ");
        }

        let _ = writeln!(self.output, "{text}");
    }

    pub fn indented<T>(&mut self, callback: impl FnOnce(&mut Self) -> T) -> T {
        self.indent += 1;
        let result = callback(self);
        self.indent -= 1;
        result
    }

}
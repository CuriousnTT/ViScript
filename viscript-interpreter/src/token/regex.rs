use std::fmt;

#[derive(Clone, PartialEq)]
pub struct RegExp {
    pub pattern: String,
    pub flags: String,
    pub source: String,
}

impl RegExp {
    pub fn new(source: String) -> Self {
        // parse source into pattern + flags
        match source.rfind('/') {
            Some(slash) if slash != 0 => Self {
                pattern: source[1..slash].to_string(),
                flags: source[slash + 1..].to_string(),
                source: source,
            },
            _ => Self {
                pattern: String::new(),
                flags: String::new(),
                source: source,
            },
        }
    }
}

impl fmt::Debug for RegExp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.source)
    }
}
impl From<String> for RegExp {
    fn from(source: String) -> Self {
        RegExp::new(source)
    }
}
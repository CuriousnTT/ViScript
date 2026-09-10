#[derive(Debug, PartialEq, Clone)]
pub enum VSX {
    Text(String), // inside elements/between tags
    String(String), // inside tags
    Identifier(String), // more permissive than normal
    Tag(Tag),

    ExpressionStart, // Any { directly inside of a tag or inside of an element.
    ExpressionEnd, // Any } that corresponds to ExpressionStart
}

impl TryFrom<&str> for VSX {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let cleaned: String = value
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();

        match cleaned.as_str() {
            "<" => Ok(VSX::Tag(Tag::OpenStart)),
            "</" => Ok(VSX::Tag(Tag::CloseStart)),
            ">" => Ok(VSX::Tag(Tag::OpenEnd)),
            "/>" => Ok(VSX::Tag(Tag::CloseEnd)),
            _ =>  Err("invalid tag delimiter"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Tag {
    OpenStart, // <
    CloseStart, // </
    OpenEnd, // >
    CloseEnd, // />
}
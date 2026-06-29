#[derive(Debug)]
pub enum LexerErrors {
    UnexpectedCharacter { character: char, position: usize },
    ParseError(String),
}

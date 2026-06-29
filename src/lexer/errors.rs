#[derive(Debug)]
pub enum LexerError {
    UnexpectedCharacter { character: char, position: usize },
    ParseError(String),
}

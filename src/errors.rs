use super::lexer::LexerError;
use std::io;

impl From<LexerError> for AppError {
    fn from(value: LexerError) -> Self {
        Self::LexerError(value)
    }
}

impl From<io::Error> for AppError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

#[derive(Debug)]
pub enum AppError {
    LexerError(LexerError),
    ParserError,
    Io(io::Error),
}

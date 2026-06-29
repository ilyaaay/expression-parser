use super::{lexer::LexerErrors, parser::ParserErrors};
use std::io;

impl From<LexerErrors> for AppError {
    fn from(value: LexerErrors) -> Self {
        Self::LexerError(value)
    }
}

impl From<ParserErrors> for AppError {
    fn from(value: ParserErrors) -> Self {
        Self::ParserError(value)
    }
}

impl From<io::Error> for AppError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

#[derive(Debug)]
pub enum AppError {
    LexerError(LexerErrors),
    ParserError(ParserErrors),
    Io(io::Error),
}

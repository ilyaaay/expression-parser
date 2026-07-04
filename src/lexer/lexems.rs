#[derive(Debug, PartialEq)]
pub enum Lexem {
    Number(u32),
    Letter(String),
    Operator(MathOperators),
    Punctuation(Punctuation),
    Brackets(Brackets),
    Comments(Comments),
    EndOfLine,
}

#[derive(Debug, PartialEq)]
pub enum Punctuation {
    Dot,
    Comma,
    Semicolon,
}

#[derive(Debug, PartialEq)]
pub enum Brackets {
    Open,
    Close,
}

#[derive(Debug, PartialEq)]
pub enum MathOperators {
    Plus,
    Minus,
    Multiplier,
    Division,
    Remainder,
    Equals,
}

#[derive(Debug, PartialEq)]
pub enum Comments {
    Line,
    Multiline,
}

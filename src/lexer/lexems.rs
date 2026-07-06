#[derive(Debug, PartialEq)]
pub enum Lexem {
    Number(Numbers),
    Letter(String),
    Operator(MathOperators),
    Punctuation(Punctuations),
    Brackets(Brackets),
    Comments(Comments),
    EndOfLine,
}

#[derive(Debug, PartialEq)]
pub enum Numbers {
    Integer(i64),
    Float(f64),
}

#[derive(Debug, PartialEq)]
pub enum Punctuations {
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

use super::Lexema;

pub struct Parser<'a>(pub &'a [Lexema]);

impl<'a> Parser<'a> {
    fn parse_math_expression(&self) {}
}

#[cfg(test)]
mod parser_tests {
    use super::{Lexema, Parser};

    #[test]
    fn test() {}
}

use super::Lexem;

pub struct Parser<I: Iterator<Item = Lexem>>(pub I);

impl<I: Iterator<Item = Lexem>> Parser<I> {}

#[cfg(test)]
mod parser_tests {
    use super::{Lexem, Parser};

    #[test]
    fn test() {}
}

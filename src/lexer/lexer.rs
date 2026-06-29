use super::errors::LexerErrors;

#[derive(Debug)]
pub struct Lexer<'a>(pub &'a str);

#[derive(Debug, PartialEq)]
pub enum Lexema {
    Number(u32),
    Letter(String),
    Operator(MathOperators),
    Punctuation(Punctuation),
    Brackets(Brackets),
}

#[derive(Debug, PartialEq)]
pub enum Punctuation {
    Dot,
    Comma,
    Semicolon,
}

#[derive(Debug, PartialEq)]
enum Brackets {
    Open,
    Close,
}

#[derive(Debug, PartialEq)]
enum MathOperators {
    Plus,
    Minus,
    Multiplier,
    Division,
    Remainder,
    Equals,
}

impl<'a> Lexer<'a> {
    pub fn get_lexems(&self) -> Result<Vec<Lexema>, LexerErrors> {
        let mut chars = self.0.char_indices().peekable();
        let mut tokens = Vec::new();

        while let Some((step, ch)) = chars.next() {
            match ch {
                ch if ch.is_ascii_whitespace() => continue,

                ch if ch.is_ascii_alphabetic() => {
                    let mut buf = String::from(ch);

                    while let Some((_, peek)) = chars.peek() {
                        if peek.is_ascii_alphabetic() {
                            buf.push(*peek);
                            chars.next();
                        } else if peek.is_ascii_whitespace() {
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    tokens.push(Lexema::Letter(buf));
                }

                '+' => tokens.push(Lexema::Operator(MathOperators::Plus)),
                '-' => tokens.push(Lexema::Operator(MathOperators::Minus)),
                '*' => tokens.push(Lexema::Operator(MathOperators::Multiplier)),
                '/' => tokens.push(Lexema::Operator(MathOperators::Division)),
                '=' => tokens.push(Lexema::Operator(MathOperators::Equals)),
                '%' => tokens.push(Lexema::Operator(MathOperators::Remainder)),

                '.' => tokens.push(Lexema::Punctuation(Punctuation::Dot)),
                ',' => tokens.push(Lexema::Punctuation(Punctuation::Comma)),
                ';' => tokens.push(Lexema::Punctuation(Punctuation::Semicolon)),

                '(' => tokens.push(Lexema::Brackets(Brackets::Open)),
                ')' => tokens.push(Lexema::Brackets(Brackets::Close)),

                ch if ch.is_ascii_digit() => {
                    let mut buf = String::new();

                    while let Some((_, peek)) = chars.peek() {
                        if peek.is_ascii_digit() {
                            buf.push(*peek);
                            chars.next();
                        } else if peek.is_ascii_whitespace() {
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    let number = buf
                        .parse::<u32>()
                        .map(Lexema::Number)
                        .map_err(|err| LexerErrors::ParseError(err.to_string()))?;

                    tokens.push(number);
                }
                _ => {
                    return Err(LexerErrors::UnexpectedCharacter {
                        character: ch,
                        position: step,
                    });
                }
            }
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod tokenizer_tests {
    use super::{Brackets, Lexema, Lexer, MathOperators};

    #[test]
    fn one_digit_numbers() {
        let lexems = Lexer("1+2").get_lexems().unwrap();
        let actual = [
            Lexema::Number(1),
            Lexema::Operator(MathOperators::Plus),
            Lexema::Number(2),
        ];
        assert_eq!(lexems, actual);

        let lexems = Lexer("\n1 + \r\t2").get_lexems().unwrap();
        let actual = [
            Lexema::Number(1),
            Lexema::Operator(MathOperators::Plus),
            Lexema::Number(2),
        ];
        assert_eq!(lexems, actual);
    }

    #[test]
    fn two_digit_numbers() {
        let lexems = Lexer("\n1\n2 - \t2\r3").get_lexems().unwrap();
        let actual = [
            Lexema::Number(12),
            Lexema::Operator(MathOperators::Minus),
            Lexema::Number(23),
        ];
        assert_eq!(lexems, actual);
    }

    #[test]
    fn letter() {
        let lexems = Lexer("var + ").get_lexems().unwrap();
        let actual = [
            Lexema::Letter("var".into()),
            Lexema::Operator(MathOperators::Plus),
        ];
        assert_eq!(lexems, actual);
    }

    #[test]
    fn two_numbes_with_commas() {
        let lexems = Lexer("( 1 + 2\n ) * 3").get_lexems().unwrap();
        let actual = [
            Lexema::Brackets(Brackets::Open),
            Lexema::Number(1),
            Lexema::Operator(MathOperators::Plus),
            Lexema::Number(2),
            Lexema::Brackets(Brackets::Close),
            Lexema::Operator(MathOperators::Multiplier),
            Lexema::Number(3),
        ];
        assert_eq!(lexems, actual);
    }
}

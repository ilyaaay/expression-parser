use std::io;

#[derive(Debug)]
pub struct Tokenizer<'a>(pub &'a str);

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(u32),
    Letter(String),
    Operator(OperatorType),
    Comma(CommaType),
}

#[derive(Debug, PartialEq)]
enum CommaType {
    Open,
    Close,
}

#[derive(Debug, PartialEq)]
enum OperatorType {
    Plus,
    Minus,
    Multiplier,
    Division,
}

#[derive(Debug)]
pub enum Error {
    UnexpectedCharacter { character: char, position: usize },
    ParseError(String),
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl<'a> Tokenizer<'a> {
    pub fn parse(&self) -> Result<Vec<Token>, Error> {
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

                    tokens.push(Token::Letter(buf));
                }

                '+' => tokens.push(Token::Operator(OperatorType::Plus)),
                '-' => tokens.push(Token::Operator(OperatorType::Minus)),
                '*' => tokens.push(Token::Operator(OperatorType::Multiplier)),
                '/' => tokens.push(Token::Operator(OperatorType::Division)),

                '(' => tokens.push(Token::Comma(CommaType::Open)),
                ')' => tokens.push(Token::Comma(CommaType::Close)),

                ch if ch.is_ascii_digit() => {
                    let mut buf = String::from(ch);

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
                        .map(Token::Number)
                        .map_err(|err| Error::ParseError(err.to_string()))?;

                    tokens.push(number);
                }
                _ => {
                    return Err(Error::UnexpectedCharacter {
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
    use super::{CommaType, OperatorType, Token, Tokenizer};

    #[test]
    fn one_digit_numbers() {
        let tokens = Tokenizer("1+2").parse().unwrap();
        let actual = [
            Token::Number(1),
            Token::Operator(OperatorType::Plus),
            Token::Number(2),
        ];
        assert_eq!(tokens, actual);

        let tokens = Tokenizer("\n1 + \r\t2").parse().unwrap();
        let actual = [
            Token::Number(1),
            Token::Operator(OperatorType::Plus),
            Token::Number(2),
        ];
        assert_eq!(tokens, actual);
    }

    #[test]
    fn two_digit_numbers() {
        let tokens = Tokenizer("\n1\n2 - \t2\r3").parse().unwrap();
        let actual = [
            Token::Number(12),
            Token::Operator(OperatorType::Minus),
            Token::Number(23),
        ];
        assert_eq!(tokens, actual);
    }

    #[test]
    fn letter() {
        let tokens = Tokenizer("var + ").parse().unwrap();
        let actual = [
            Token::Letter("var".into()),
            Token::Operator(OperatorType::Plus),
        ];
        assert_eq!(tokens, actual);
    }

    #[test]
    fn two_numbes_with_commas() {
        let tokens = Tokenizer("( 1 + 2\n ) * 3").parse().unwrap();
        let actual = [
            Token::Comma(CommaType::Open),
            Token::Number(1),
            Token::Operator(OperatorType::Plus),
            Token::Number(2),
            Token::Comma(CommaType::Close),
            Token::Operator(OperatorType::Multiplier),
            Token::Number(3),
        ];
        assert_eq!(tokens, actual);
    }
}

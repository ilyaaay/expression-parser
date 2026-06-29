use super::errors::LexerError;

#[derive(Debug)]
pub struct Lexer<'a>(pub &'a str);

#[derive(Debug, PartialEq)]
pub enum Lexema {
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

impl<'a> Lexer<'a> {
    pub fn get_lexems(&self) -> Result<Vec<Lexema>, LexerError> {
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

                '+' => tokens.push(Lexema::Operator(OperatorType::Plus)),
                '-' => tokens.push(Lexema::Operator(OperatorType::Minus)),
                '*' => tokens.push(Lexema::Operator(OperatorType::Multiplier)),
                '/' => tokens.push(Lexema::Operator(OperatorType::Division)),

                '(' => tokens.push(Lexema::Comma(CommaType::Open)),
                ')' => tokens.push(Lexema::Comma(CommaType::Close)),

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
                        .map(Lexema::Number)
                        .map_err(|err| LexerError::ParseError(err.to_string()))?;

                    tokens.push(number);
                }
                _ => {
                    return Err(LexerError::UnexpectedCharacter {
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
    use super::{CommaType, Lexema, Lexer, OperatorType};

    #[test]
    fn one_digit_numbers() {
        let lexems = Lexer("1+2").get_lexems().unwrap();
        let actual = [
            Lexema::Number(1),
            Lexema::Operator(OperatorType::Plus),
            Lexema::Number(2),
        ];
        assert_eq!(lexems, actual);

        let lexems = Lexer("\n1 + \r\t2").get_lexems().unwrap();
        let actual = [
            Lexema::Number(1),
            Lexema::Operator(OperatorType::Plus),
            Lexema::Number(2),
        ];
        assert_eq!(lexems, actual);
    }

    #[test]
    fn two_digit_numbers() {
        let lexems = Lexer("\n1\n2 - \t2\r3").get_lexems().unwrap();
        let actual = [
            Lexema::Number(12),
            Lexema::Operator(OperatorType::Minus),
            Lexema::Number(23),
        ];
        assert_eq!(lexems, actual);
    }

    #[test]
    fn letter() {
        let lexems = Lexer("var + ").get_lexems().unwrap();
        let actual = [
            Lexema::Letter("var".into()),
            Lexema::Operator(OperatorType::Plus),
        ];
        assert_eq!(lexems, actual);
    }

    #[test]
    fn two_numbes_with_commas() {
        let lexems = Lexer("( 1 + 2\n ) * 3").get_lexems().unwrap();
        let actual = [
            Lexema::Comma(CommaType::Open),
            Lexema::Number(1),
            Lexema::Operator(OperatorType::Plus),
            Lexema::Number(2),
            Lexema::Comma(CommaType::Close),
            Lexema::Operator(OperatorType::Multiplier),
            Lexema::Number(3),
        ];
        assert_eq!(lexems, actual);
    }
}

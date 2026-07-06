use super::{
    errors::LexerErrors,
    lexems::{Brackets, Comments, Lexem, MathOperators, Punctuations},
};

pub struct Lexer<I: Iterator<Item = char>>(pub I);

impl<I: Iterator<Item = char>> Lexer<I> {
    pub fn get_lexems(self) -> Result<Vec<Lexem>, LexerErrors> {
        let mut chars = self.0.peekable();
        let mut tokens = Vec::new();
        let mut position = 0;

        while let Some(ch) = chars.next() {
            position += 1;

            match ch {
                '\t' | '\x0C' | '\r' | ' ' => continue,
                '\n' => tokens.push(Lexem::EndOfLine),

                ch if ch.is_ascii_alphabetic() => {
                    let mut buf = String::from(ch);

                    while let Some(peek) = chars.peek() {
                        if peek.is_ascii_alphabetic() {
                            buf.push(*peek);
                            chars.next();
                        } else if peek.is_ascii_whitespace() {
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    tokens.push(Lexem::Letter(buf));
                }

                '+' => tokens.push(Lexem::Operator(MathOperators::Plus)),
                '-' => tokens.push(Lexem::Operator(MathOperators::Minus)),
                '*' => tokens.push(Lexem::Operator(MathOperators::Multiplier)),
                '=' => tokens.push(Lexem::Operator(MathOperators::Equals)),
                '%' => tokens.push(Lexem::Operator(MathOperators::Remainder)),

                '/' => {
                    if let Some(next) = chars.next()
                        && next == '/'
                    {
                        if let Some(peek) = chars.peek()
                            && *peek == '/'
                        {
                            tokens.push(Lexem::Comments(Comments::Multiline));
                            chars.next();
                        } else {
                            tokens.push(Lexem::Comments(Comments::Line));
                        }
                    } else {
                        tokens.push(Lexem::Operator(MathOperators::Division));
                    }
                }

                '.' => tokens.push(Lexem::Punctuation(Punctuations::Dot)),
                ',' => tokens.push(Lexem::Punctuation(Punctuations::Comma)),
                ';' => tokens.push(Lexem::Punctuation(Punctuations::Semicolon)),

                '(' => tokens.push(Lexem::Brackets(Brackets::Open)),
                ')' => tokens.push(Lexem::Brackets(Brackets::Close)),

                ch if ch.is_ascii_digit() => {
                    let mut buf = String::from(ch);
                    let mut is_float = false;

                    while let Some(&peek) = chars.peek() {
                        if peek.is_ascii_digit() && !is_float {
                            buf.push(peek);
                            chars.next();
                        } else if peek.is_ascii_whitespace() {
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    let number = buf
                        .parse::<u32>()
                        .map(Lexem::Number)
                        .map_err(|err| LexerErrors::ParseError(err.to_string()))?;

                    tokens.push(number);
                }
                _ => {
                    return Err(LexerErrors::UnexpectedCharacter {
                        character: ch,
                        position,
                    });
                }
            }
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod tokenizer_tests {
    use crate::lexer::lexems::Numbers;

    use super::{Brackets, Lexem, Lexer, MathOperators};

    #[test]
    fn one_digit_numbers() {
        let lexems = Lexer("1+2".chars()).get_lexems().unwrap();
        let actual = [
            Lexem::Number(1),
            Lexem::Operator(MathOperators::Plus),
            Lexem::Number(2),
        ];
        assert_eq!(lexems, actual);

        let lexems = Lexer("\n1 + \r\t2".chars()).get_lexems().unwrap();
        let actual = [
            Lexem::Number(1),
            Lexem::Operator(MathOperators::Plus),
            Lexem::Number(2),
        ];
        assert_eq!(lexems, actual);
    }

    #[test]
    fn two_digit_numbers() {
        let lexems = Lexer("\n1\n2 - \t2\r3".chars()).get_lexems().unwrap();
        let actual = [
            Lexem::Number(Numbers::Integer(12)),
            Lexem::Operator(MathOperators::Minus),
            Lexem::Number(Numbers::Integer(1)),
        ];
        assert_eq!(lexems, actual);
    }

    #[test]
    fn letter() {
        let lexems = Lexer("var + ".chars()).get_lexems().unwrap();
        let actual = [
            Lexem::Letter("var".into()),
            Lexem::Operator(MathOperators::Plus),
        ];
        assert_eq!(lexems, actual);
    }

    #[test]
    fn two_numbes_with_commas() {
        let lexems = Lexer("( 1 + 2\n ) * 3".chars()).get_lexems().unwrap();
        let actual = [
            Lexem::Brackets(Brackets::Open),
            Lexem::Number(Numbers::Integer(1)),
            Lexem::Operator(MathOperators::Plus),
            Lexem::Number(Numbers::Integer(2)),
            Lexem::Brackets(Brackets::Close),
            Lexem::Operator(MathOperators::Multiplier),
            Lexem::Number(Numbers::Integer(3)),
        ];
        assert_eq!(lexems, actual);
    }

    #[test]
    fn division() {
        let lexems = Lexer("1 / 2 = 0.5".chars()).get_lexems().unwrap();
        let actual = [
            Lexem::Number(Numbers::Integer(1)),
            Lexem::Operator(MathOperators::Division),
            Lexem::Number(Numbers::Integer(2)),
            Lexem::Operator(MathOperators::Equals),
        ];
    }
}

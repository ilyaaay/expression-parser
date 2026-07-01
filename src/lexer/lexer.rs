use super::{
    errors::LexerErrors,
    models::{Brackets, Comments, Lexema, MathOperators, Punctuation},
};

pub struct Lexer<'a>(pub std::str::Chars<'a>);

impl<'a> Lexer<'a> {
    pub fn get_lexems(self) -> Result<Vec<Lexema>, LexerErrors> {
        let mut chars = self.0.enumerate().peekable();
        let mut tokens = Vec::new();

        while let Some((step, ch)) = chars.next() {
            match ch {
                '\t' | '\x0C' | '\r' | ' ' => continue,
                '\n' => tokens.push(Lexema::EndOfLine),

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
                '=' => tokens.push(Lexema::Operator(MathOperators::Equals)),
                '%' => tokens.push(Lexema::Operator(MathOperators::Remainder)),

                '/' => {
                    if let Some((_, next)) = chars.next()
                        && next == '/'
                    {
                        if let Some((_, peek)) = chars.peek()
                            && *peek == '/'
                        {
                            tokens.push(Lexema::Comments(Comments::Multiline));
                            chars.next();
                        } else {
                            tokens.push(Lexema::Comments(Comments::Line));
                        }
                    } else {
                        tokens.push(Lexema::Operator(MathOperators::Division));
                    }
                }

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
        let lexems = Lexer("1+2".chars()).get_lexems().unwrap();
        let actual = [
            Lexema::Number(1),
            Lexema::Operator(MathOperators::Plus),
            Lexema::Number(2),
        ];
        assert_eq!(lexems, actual);

        let lexems = Lexer("\n1 + \r\t2".chars()).get_lexems().unwrap();
        let actual = [
            Lexema::Number(1),
            Lexema::Operator(MathOperators::Plus),
            Lexema::Number(2),
        ];
        assert_eq!(lexems, actual);
    }

    #[test]
    fn two_digit_numbers() {
        let lexems = Lexer("\n1\n2 - \t2\r3".chars()).get_lexems().unwrap();
        let actual = [
            Lexema::Number(12),
            Lexema::Operator(MathOperators::Minus),
            Lexema::Number(23),
        ];
        assert_eq!(lexems, actual);
    }

    #[test]
    fn letter() {
        let lexems = Lexer("var + ".chars()).get_lexems().unwrap();
        let actual = [
            Lexema::Letter("var".into()),
            Lexema::Operator(MathOperators::Plus),
        ];
        assert_eq!(lexems, actual);
    }

    #[test]
    fn two_numbes_with_commas() {
        let lexems = Lexer("( 1 + 2\n ) * 3".chars()).get_lexems().unwrap();
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

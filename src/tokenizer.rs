#[derive(Debug, PartialEq)]
pub enum Token {
    Arrow,
    Boolean(bool),
    Divide,
    Equals,
    Identifier(String),
    Integer(i32),
    KeywordElse,
    KeywordEnd,
    KeywordFn,
    KeywordIf,
    KeywordIn,
    KeywordLet,
    KeywordThen,
    KeywordVal,
    LeftParenthesis,
    Minus,
    Plus,
    RightParenthesis,
    Times,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokenizer = Tokenizer::new(input);
    tokenizer.tokenize()
}

struct Tokenizer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Tokenizer<'a> {
    fn new(input: &str) -> Tokenizer {
        Tokenizer { input, position: 0 }
    }

    fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token()? {
            tokens.push(token);
        }
        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Option<Token>, String> {
        self.skip_whitespace();
        match self.next_char() {
            Some(c) => {
                if c.is_alphabetic() {
                    Ok(Some(self.read_identifier()))
                } else if c.is_numeric() {
                    Ok(Some(self.read_integer()))
                } else {
                    match c {
                        '(' => {
                            self.position += 1;
                            Ok(Some(Token::LeftParenthesis))
                        }
                        ')' => {
                            self.position += 1;
                            Ok(Some(Token::RightParenthesis))
                        }
                        '+' => {
                            self.position += 1;
                            Ok(Some(Token::Plus))
                        }
                        '-' => {
                            if let Some(c2) = self.peek_char() {
                                if c2.is_numeric() {
                                    return Ok(Some(self.read_integer()));
                                }
                            }
                            self.position += 1;
                            Ok(Some(Token::Minus))
                        }
                        '*' => {
                            self.position += 1;
                            Ok(Some(Token::Times))
                        }
                        '/' => {
                            self.position += 1;
                            Ok(Some(Token::Divide))
                        }
                        '=' => {
                            if let Some(c2) = self.peek_char() {
                                if c2 == '>' {
                                    self.position += 2;
                                    return Ok(Some(Token::Arrow));
                                }
                            }
                            self.position += 1;
                            Ok(Some(Token::Equals))
                        }
                        _ => Err(format!("unexpected character: {}", c)),
                    }
                }
            }
            None => Ok(None),
        }
    }

    fn is_eof(&self) -> bool {
        self.position >= self.input.len()
    }

    fn next_char(&self) -> Option<char> {
        if self.is_eof() {
            None
        } else {
            self.input[self.position..].chars().next()
        }
    }

    fn peek_char(&self) -> Option<char> {
        if self.position + 1 >= self.input.len() {
            None
        } else {
            self.input[self.position + 1..].chars().next()
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.next_char() {
            if c.is_whitespace() {
                self.position += 1;
            } else {
                break;
            }
        }
    }

    fn read_identifier(&mut self) -> Token {
        let mut buffer = String::new();
        while let Some(c) = self.next_char() {
            if c.is_alphabetic() {
                buffer.push(c);
                self.position += 1;
            } else {
                break;
            }
        }
        match buffer.as_str() {
            "true" => Token::Boolean(true),
            "false" => Token::Boolean(false),
            "else" => Token::KeywordElse,
            "end" => Token::KeywordEnd,
            "fn" => Token::KeywordFn,
            "if" => Token::KeywordIf,
            "in" => Token::KeywordIn,
            "let" => Token::KeywordLet,
            "then" => Token::KeywordThen,
            "val" => Token::KeywordVal,
            _ => Token::Identifier(buffer),
        }
    }

    fn read_integer(&mut self) -> Token {
        let mut buffer = String::new();
        if let Some(c) = self.next_char() {
            if c == '-' {
                buffer.push(c);
                self.position += 1;
            }
        }
        while let Some(c) = self.next_char() {
            if c.is_numeric() {
                buffer.push(c);
                self.position += 1;
            } else {
                break;
            }
        }
        let value: i32 = buffer.parse().unwrap();
        Token::Integer(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::{tokenize, Token};

    #[test]
    fn test_tokenize_boolean_true() {
        assert_eq!(tokenize("true"), Ok(vec![Token::Boolean(true)]));
    }

    #[test]
    fn test_tokenize_boolean_false() {
        assert_eq!(tokenize("false"), Ok(vec![Token::Boolean(false)]));
    }

    #[test]
    fn test_tokenize_single_character_identifier() {
        assert_eq!(
            tokenize("p"),
            Ok(vec![Token::Identifier(String::from("p"))])
        );
    }

    #[test]
    fn test_tokenize_multi_character_identifier() {
        assert_eq!(
            tokenize("pi"),
            Ok(vec![Token::Identifier(String::from("pi"))])
        );
    }

    #[test]
    fn test_tokenize_single_digit_integer() {
        assert_eq!(tokenize("4"), Ok(vec![Token::Integer(4)]));
    }

    #[test]
    fn test_tokenize_multi_digit_integer() {
        assert_eq!(tokenize("42"), Ok(vec![Token::Integer(42)]));
    }

    #[test]
    fn test_tokenize_negative_integer() {
        assert_eq!(tokenize("-42"), Ok(vec![Token::Integer(-42)]));
    }

    #[test]
    fn test_tokenize_minus_and_integer() {
        assert_eq!(tokenize("- 42"), Ok(vec![Token::Minus, Token::Integer(42)]));
    }

    #[test]
    fn test_tokenize_math_operators() {
        assert_eq!(
            tokenize("+ - * / ="),
            Ok(vec![
                Token::Plus,
                Token::Minus,
                Token::Times,
                Token::Divide,
                Token::Equals,
            ])
        );
    }

    #[test]
    fn test_tokenize_parentheses() {
        assert_eq!(
            tokenize("()"),
            Ok(vec![Token::LeftParenthesis, Token::RightParenthesis])
        );
    }

    #[test]
    fn test_tokenize_arrow() {
        assert_eq!(tokenize("=>"), Ok(vec![Token::Arrow]));
    }

    #[test]
    fn test_tokenize_with_leading_and_trailing_whitespace() {
        assert_eq!(tokenize(" 42 "), Ok(vec![Token::Integer(42)]));
    }

    #[test]
    fn test_tokenize_multiline_string() {
        assert_eq!(
            tokenize("1 +\n2"),
            Ok(vec![Token::Integer(1), Token::Plus, Token::Integer(2)])
        );
    }

    #[test]
    fn test_tokenize_function_definition() {
        assert_eq!(
            tokenize("fn x => x + 1"),
            Ok(vec![
                Token::KeywordFn,
                Token::Identifier(String::from("x")),
                Token::Arrow,
                Token::Identifier(String::from("x")),
                Token::Plus,
                Token::Integer(1),
            ])
        );
    }

    #[test]
    fn test_tokenize_if_expression() {
        assert_eq!(
            tokenize("if x = y then 0 else 1"),
            Ok(vec![
                Token::KeywordIf,
                Token::Identifier(String::from("x")),
                Token::Equals,
                Token::Identifier(String::from("y")),
                Token::KeywordThen,
                Token::Integer(0),
                Token::KeywordElse,
                Token::Integer(1)
            ])
        );
    }

    #[test]
    fn test_tokenize_let_expression() {
        assert_eq!(
            tokenize("let val inc = fn x => x + 1 in inc(42) end"),
            Ok(vec![
                Token::KeywordLet,
                Token::KeywordVal,
                Token::Identifier(String::from("inc")),
                Token::Equals,
                Token::KeywordFn,
                Token::Identifier(String::from("x")),
                Token::Arrow,
                Token::Identifier(String::from("x")),
                Token::Plus,
                Token::Integer(1),
                Token::KeywordIn,
                Token::Identifier(String::from("inc")),
                Token::LeftParenthesis,
                Token::Integer(42),
                Token::RightParenthesis,
                Token::KeywordEnd,
            ])
        );
    }
}

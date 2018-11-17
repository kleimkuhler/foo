#![allow(dead_code)]

use std::iter::Peekable;

fn is_symbol(ch: char) -> bool {
    match ch {
        '(' | ')' | '{' | '}' => true,
        '/' | '*' | '+' | '-' => true,

        _ => false,
    }
}

#[derive(Debug, PartialEq)]
enum Value {
    Integer(i64),
    String(String),
}

#[derive(Debug, PartialEq)]
enum Token {
    Value(Value),

    LParen,
    RParen,
    LBracket,
    RBracket,

    Divide,
    Multiply,
    Plus,
    Subtract,
}

struct Tokenizer<I: Iterator<Item = char>> {
    chars: Peekable<I>,
}

impl<I: Iterator<Item = char>> Tokenizer<I> {
    fn next(&mut self) -> Option<Token> {
        let ch = match self.peek() {
            Some(ch) => ch,
            None => return None,
        };

        if is_symbol(ch) {
            match self.chars.next() {
                Some('/') => return Some(Token::Divide),
                Some('*') => return Some(Token::Multiply),
                Some('+') => return Some(Token::Plus),
                Some('-') => return Some(Token::Subtract),
                Some('(') => return Some(Token::LParen),
                Some(')') => return Some(Token::RParen),
                Some('{') => return Some(Token::LBracket),
                Some('}') => return Some(Token::RBracket),

                _ => (),
            }
        }

        if ch == '\'' {
            self.chars.next();
            let string = self.read_while(|ch| ch != '\'');
            return Some(Token::Value(Value::String(string)));
        }

        let string = self.read_while(|ch| !ch.is_whitespace() && ch != '\n');
        match ch {
            '0'...'9' => {
                if let Ok(num) = string.parse::<i64>() {
                    return Some(Token::Value(Value::Integer(num)));
                }

                return None;
            }

            _ => None,
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.trim();
        self.chars.peek().cloned()
    }

    fn read_while<F>(&mut self, mut proceed: F) -> String
    where
        F: FnMut(char) -> bool,
    {
        let mut string = String::new();
        while let Some(ch) = self.chars.next() {
            if proceed(ch) {
                string.push(ch)
            } else {
                return string;
            }
        }

        return string;
    }

    fn trim(&mut self) {
        loop {
            match self.chars.peek().cloned() {
                Some(ch) if ch.is_whitespace() => {
                    self.chars.next();
                }

                _ => break,
            }
        }
    }
}

fn tokenizer(input: &str) -> Vec<Token> {
    let mut tokenizer = Tokenizer {
        chars: input.chars().peekable(),
    };

    let mut tokens = Vec::new();
    while let Some(token) = tokenizer.next() {
        tokens.push(token)
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world_string() {
        assert_eq!(
            tokenizer("'hello, world!'"),
            &[Token::Value(Value::String("hello, world!".to_string()))]
        )
    }

    #[test]
    fn multiple_line_string() {
        assert_eq!(
            tokenizer(
                "'multiple
            lines!'"
            ),
            &[Token::Value(Value::String(
                "multiple
            lines!"
                    .to_string()
            ))]
        )
    }

    #[test]
    fn single_digit() {
        assert_eq!(tokenizer("1"), &[Token::Value(Value::Integer(1))])
    }

    #[test]
    fn multiple_digits() {
        assert_eq!(tokenizer("123"), &[Token::Value(Value::Integer(123))])
    }

    #[test]
    fn multiple_values() {
        assert_eq!(
            tokenizer("'hello' 123 '!'"),
            &[
                Token::Value(Value::String("hello".to_string())),
                Token::Value(Value::Integer(123)),
                Token::Value(Value::String("!".to_string()))
            ]
        )
    }

    #[test]
    fn delimiters() {
        assert_eq!(
            tokenizer("((){{}})"),
            &[
                Token::LParen,
                Token::LParen,
                Token::RParen,
                Token::LBracket,
                Token::LBracket,
                Token::RBracket,
                Token::RBracket,
                Token::RParen
            ]
        )
    }

    #[test]
    fn arithmetic_symbols() {
        assert_eq!(
            tokenizer("/ * + -"),
            &[Token::Divide, Token::Multiply, Token::Plus, Token::Subtract]
        )
    }

    #[test]
    fn simple_arithmetic_expression() {
        assert_eq!(
            tokenizer("1 + 2 * 3 - 4"),
            &[
                Token::Value(Value::Integer(1)),
                Token::Plus,
                Token::Value(Value::Integer(2)),
                Token::Multiply,
                Token::Value(Value::Integer(3)),
                Token::Subtract,
                Token::Value(Value::Integer(4))
            ]
        )
    }
}

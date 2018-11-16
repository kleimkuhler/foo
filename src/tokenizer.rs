#![allow(dead_code)]

use std::iter::Peekable;

#[derive(Debug, PartialEq)]
enum Value {
    Integer(u32),
    String(String),
}

#[derive(Debug, PartialEq)]
enum Token {
    Value(Value),
}

struct Tokenizer<I: Iterator<Item = char>> {
    chars: Peekable<I>,
}

impl<I: Iterator<Item = char>> Tokenizer<I> {
    fn next(&mut self) -> Option<Token> {
        let c = match self.peek() {
            Some(c) => c,
            None => return None,
        };

        self.chars.next();
        match c {
            '\'' => {
                let mut string = String::new();
                while let Some(c) = self.chars.next() {
                    if c != '\'' {
                        string.push(c)
                    }
                }

                return Some(Token::Value(Value::String(string)));
            }
            '0'...'9' => {
                if let Some(digit) = c.to_digit(10) {
                    return Some(Token::Value(Value::Integer(digit)));
                }

                return None;
            }
            _ => None,
        }
    }

    fn peek(&mut self) -> Option<char> {
        let c = self.chars.peek();
        c.cloned()
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
        assert_eq!(tokenizer("1"), &[Token::Value(Value::Integer(1))]);
    }
}

#![allow(dead_code)]

use std::{iter::Peekable, result::Result as StdResult};

fn is_symbol(ch: char) -> bool {
    match ch {
        '(' | ')' | '\\' | '.' => true,

        _ => false,
    }
}

fn is_ident(ch: char) -> bool {
    match ch {
        'a'...'z' | 'A'...'Z' | '_' => true,

        _ => false,
    }
}

#[derive(Debug, PartialEq)]
enum LexerError {
    InvalidSymbol(char),
    UnknownToken,
}

type Result<T> = StdResult<T, LexerError>;

#[derive(Debug, PartialEq)]
enum Token {
    // Data
    Ident(String),

    // Delimiters
    LParen,
    RParen,

    // Symbols
    BSlash,
    Dot,
}

struct Tokenizer<I: Iterator<Item = char>> {
    chars: Peekable<I>,
}

impl<I: Iterator<Item = char>> Tokenizer<I> {
    fn next(&mut self) -> Result<Option<Token>> {
        while let Some(ch) = self.peek() {
            if is_symbol(ch) {
                self.chars.next();

                return match ch {
                    '(' => Ok(Some(Token::LParen)),
                    ')' => Ok(Some(Token::RParen)),
                    '\\' => Ok(Some(Token::BSlash)),
                    '.' => Ok(Some(Token::Dot)),

                    _ => Err(LexerError::InvalidSymbol(ch)),
                };
            }

            if is_ident(ch) {
                let ident = self.read_while(|ch| is_ident(ch));

                return Ok(Some(Token::Ident(ident)));
            }

            if ch.is_whitespace() {
                self.chars.next();
                continue;
            }

            return Err(LexerError::UnknownToken);
        }

        Ok(None)
    }

    fn read_while<F>(&mut self, mut proceed: F) -> String
    where
        F: FnMut(char) -> bool,
    {
        let mut string = String::new();
        while let Some(ch) = self.peek() {
            if proceed(ch) {
                self.chars.next();
                string.push(ch)
            } else {
                return string;
            }
        }

        return string;
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().cloned()
    }
}

fn tokenizer(input: &str) -> Result<Vec<Token>> {
    let mut tokenizer = Tokenizer {
        chars: input.chars().peekable(),
    };

    let mut tokens = Vec::new();
    while let Some(token) = tokenizer.next()? {
        tokens.push(token)
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity() {
        assert_eq!(
            tokenizer("foo").unwrap(),
            &[Token::Ident("foo".to_string())]
        )
    }

    #[test]
    fn identities() {
        assert_eq!(
            tokenizer("foo bar baz").unwrap(),
            &[
                Token::Ident("foo".to_string()),
                Token::Ident("bar".to_string()),
                Token::Ident("baz".to_string())
            ]
        )
    }

    #[test]
    fn symbols() {
        assert_eq!(
            tokenizer(r"( ( . \ ( \ ) . ) )").unwrap(),
            &[
                Token::LParen,
                Token::LParen,
                Token::Dot,
                Token::BSlash,
                Token::LParen,
                Token::BSlash,
                Token::RParen,
                Token::Dot,
                Token::RParen,
                Token::RParen
            ]
        )
    }

    #[test]
    fn program_1() {
        assert_eq!(
            tokenizer(r"\ x . x").unwrap(),
            &[
                Token::BSlash,
                Token::Ident("x".to_string()),
                Token::Dot,
                Token::Ident("x".to_string())
            ]
        )
    }

    #[test]
    fn program_2() {
        assert_eq!(
            tokenizer(r"(\ foo . \ y . foo y) (\ bar . bar) w").unwrap(),
            &[
                Token::LParen,
                Token::BSlash,
                Token::Ident("foo".to_string()),
                Token::Dot,
                Token::BSlash,
                Token::Ident("y".to_string()),
                Token::Dot,
                Token::Ident("foo".to_string()),
                Token::Ident("y".to_string()),
                Token::RParen,
                Token::LParen,
                Token::BSlash,
                Token::Ident("bar".to_string()),
                Token::Dot,
                Token::Ident("bar".to_string()),
                Token::RParen,
                Token::Ident("w".to_string()),
            ]
        )
    }
}

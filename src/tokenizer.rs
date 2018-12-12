#![allow(dead_code)]

use std::{fmt, iter::Peekable, result::Result as StdResult, str::CharIndices};

fn is_identity(ch: char) -> bool {
    match ch {
        'a'...'z' | 'A'...'Z' | '_' => true,
        _ => false,
    }
}

#[derive(Debug, PartialEq)]
enum LexerError {
    UnexpectedCharacter(char),
}

type Result<T, E> = StdResult<T, E>;

#[derive(Debug, PartialEq)]
pub enum Token<'input> {
    // Data
    Identity(&'input str),

    // Symbols
    BackSlash,
    Dot,
    LeftParen,
    RightParen,
}

impl<'input> fmt::Display for Token<'input> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Identity(ident) => write!(f, "{}", ident),
            Token::BackSlash => write!(f, "\\"),
            Token::Dot => write!(f, "."),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
        }
    }
}

struct Tokenizer<'input> {
    input: &'input str,
    chars: Peekable<CharIndices<'input>>,
}

impl<'input> Tokenizer<'input> {
    fn new(input: &'input str) -> Self {
        Tokenizer {
            input,
            chars: input.char_indices().peekable(),
        }
    }

    fn read_while<F>(&mut self, start: usize, mut proceed: F) -> &'input str
    where
        F: FnMut(char) -> bool,
    {
        while let Some((end, ch)) = self.chars.peek().cloned() {
            if proceed(ch) {
                self.chars.next();
            } else {
                return &self.input[start..end];
            }
        }

        &self.input[start..self.input.len()]
    }
}

impl<'input> Iterator for Tokenizer<'input> {
    type Item = Result<Token<'input>, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((i, ch)) = self.chars.next() {
            return Some(match ch {
                '(' => Ok(Token::LeftParen),
                ')' => Ok(Token::RightParen),
                '\\' => Ok(Token::BackSlash),
                '.' => Ok(Token::Dot),
                ch if is_identity(ch) => {
                    let identity = self.read_while(i, |ch| is_identity(ch));
                    Ok(Token::Identity(identity))
                }
                ch if ch.is_whitespace() => continue,
                _ => Err(LexerError::UnexpectedCharacter(ch)),
            });
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokenize(input: &str) -> Vec<Result<Token, LexerError>> {
        let mut tokenizer = Tokenizer::new(input);
        let mut tokens = Vec::new();

        while let Some(token) = tokenizer.next() {
            tokens.push(token)
        }

        tokens
    }

    #[test]
    fn identities() {
        assert_eq!(
            tokenize("foo bar baz"),
            &[
                Ok(Token::Identity("foo")),
                Ok(Token::Identity("bar")),
                Ok(Token::Identity("baz"))
            ]
        )
    }

    #[test]
    fn symbols() {
        assert_eq!(
            tokenize(r"( ( . \ ( \ ) . ) )"),
            &[
                Ok(Token::LeftParen),
                Ok(Token::LeftParen),
                Ok(Token::Dot),
                Ok(Token::BackSlash),
                Ok(Token::LeftParen),
                Ok(Token::BackSlash),
                Ok(Token::RightParen),
                Ok(Token::Dot),
                Ok(Token::RightParen),
                Ok(Token::RightParen)
            ]
        )
    }

    #[test]
    fn program() {
        assert_eq!(
            tokenize(r"(\ foo . \ y . foo y) (\ bar . bar) w"),
            &[
                Ok(Token::LeftParen),
                Ok(Token::BackSlash),
                Ok(Token::Identity("foo")),
                Ok(Token::Dot),
                Ok(Token::BackSlash),
                Ok(Token::Identity("y")),
                Ok(Token::Dot),
                Ok(Token::Identity("foo")),
                Ok(Token::Identity("y")),
                Ok(Token::RightParen),
                Ok(Token::LeftParen),
                Ok(Token::BackSlash),
                Ok(Token::Identity("bar")),
                Ok(Token::Dot),
                Ok(Token::Identity("bar")),
                Ok(Token::RightParen),
                Ok(Token::Identity("w"))
            ]
        )
    }
}

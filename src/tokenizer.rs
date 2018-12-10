#![allow(dead_code)]

use std::{fmt, iter::Peekable, result::Result as StdResult, str::CharIndices};

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
    UnexpectedCharacter(char),
}

type Result<T, E> = StdResult<T, E>;

#[derive(Debug, PartialEq)]
pub enum Token<'input> {
    // Data
    Identifier(&'input str),

    // Symbols
    BackSlash,
    Dot,
    LeftParen,
    RightParen,
}

impl<'input> fmt::Display for Token<'input> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Identifier(ident) => write!(f, "{}", ident),
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
                ch if is_symbol(ch) => {
                    self.chars.next();

                    match ch {
                        '(' => Ok(Token::LeftParen),
                        ')' => Ok(Token::LeftParen),
                        '\\' => Ok(Token::LeftParen),
                        '.' => Ok(Token::LeftParen),

                        _ => Err(LexerError::UnexpectedCharacter(ch)),
                    }
                }
                ch if is_ident(ch) => {
                    let ident = self.read_while(i, |ch| is_ident(ch));
                    Ok(Token::Identifier(ident))
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
    fn identity() {
        assert_eq!(tokenize("foo"), &[Ok(Token::Identifier("foo"))])
    }

    #[test]
    fn identities() {
        assert_eq!(
            tokenize("foo bar baz"),
            &[
                Ok(Token::Identifier("foo")),
                Ok(Token::Identifier("bar")),
                Ok(Token::Identifier("baz"))
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

    // #[test]
    // fn program_1() {
    //     assert_eq!(
    //         tokenizer(r"\ x . x").unwrap(),
    //         &[
    //             Token::BackSlash,
    //             Token::Identifier("x".to_string()),
    //             Token::Dot,
    //             Token::Identifier("x".to_string())
    //         ]
    //     )
    // }

    // #[test]
    // fn program_2() {
    //     assert_eq!(
    //         tokenizer(r"(\ foo . \ y . foo y) (\ bar . bar) w").unwrap(),
    //         &[
    //             Token::LeftParen,
    //             Token::BackSlash,
    //             Token::Identifier("foo".to_string()),
    //             Token::Dot,
    //             Token::BackSlash,
    //             Token::Identifier("y".to_string()),
    //             Token::Dot,
    //             Token::Identifier("foo".to_string()),
    //             Token::Identifier("y".to_string()),
    //             Token::RightParen,
    //             Token::LeftParen,
    //             Token::BackSlash,
    //             Token::Identifier("bar".to_string()),
    //             Token::Dot,
    //             Token::Identifier("bar".to_string()),
    //             Token::RightParen,
    //             Token::Identifier("w".to_string()),
    //         ]
    //     )
    // }
}

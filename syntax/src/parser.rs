#![allow(dead_code)]
use crate::tokenizer::{tokenize, LexerError, Token, Tokenizer};
use std::{iter::Peekable, result::Result as StdResult};

type Result<T, E> = StdResult<T, E>;

#[derive(Debug, PartialEq)]
enum ParserError {}

#[derive(Debug, PartialEq)]
enum Term {
    // Core lambda calculus forms
    Ref(String),
    Lambda(Vec<String>, Box<Term>),
    App(Box<Term>, Box<Term>),

    // Scheme forms
    Bool(bool),
    If(Box<Term>, Box<Term>, Box<Term>),
    And(Box<Term>, Box<Term>),
    Or(Box<Term>, Box<Term>),

    // Numerics
    Int(i64),
}

struct Parser<'input> {
    tokens: Peekable<Tokenizer<'input>>,
}

impl<'input> Parser<'input> {
    fn new(tokenizer: Tokenizer<'input>) -> Self {
        Parser {
            tokens: tokenizer.peekable(),
        }
    }
}

impl<'input> Iterator for Parser<'input> {
    type Item = Result<Term, ParserError>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(Ok(token)) = self.tokens.next() {
            return Some(match token {
                Token::Identifier(_) => Ok(Term::Int(0)),
                Token::StringLiteral(_) => Ok(Term::Int(0)),
                Token::LBracket => Ok(Term::Int(0)),
                Token::RBracket => Ok(Term::Int(0)),
                Token::LCurlyBrace => Ok(Term::Int(0)),
                Token::RCurlyBrace => Ok(Term::Int(0)),
                Token::LParen => Ok(Term::Int(0)),
                Token::RParen => Ok(Term::Int(0)),
            });
        }

        None
    }
}

fn parse(input: Tokenizer) -> Vec<Result<Term, ParserError>> {
    Parser::new(input).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let tokens = Tokenizer::new("(0)");
        assert_eq!(
            parse(tokens),
            &[Ok(Term::Int(0)), Ok(Term::Int(0)), Ok(Term::Int(0))]
        )
    }
}

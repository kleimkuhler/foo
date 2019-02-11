#![allow(dead_code)]
use crate::tokenizer::Tokenizer;
use std::{iter::Peekable, result::Result as StdResult};

type Result<T, E> = StdResult<T, E>;

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

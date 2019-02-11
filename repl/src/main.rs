extern crate syntax;

use std::io::{self, Error};
use syntax::tokenizer::{LexerError, Token, Tokenizer};

fn main() -> Result<(), Error> {
    loop {
        let mut buffer = String::new();
        let stdin = io::stdin();

        stdin.read_line(&mut buffer)?;
        let tokens: Vec<Result<Token<'_>, LexerError>> = Tokenizer::new(&buffer).collect();
        println!("{:?}", tokens)
    }
}

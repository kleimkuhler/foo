// use tokenizer::Token;

// use std::{iter::Peekable, result::Result as StdResult};

// struct ParserError;

// type Result<T> = StdResult<T, ParserError>;

// struct Ident(String);

// enum Ast {
//     Var(Ident),
//     Abs(Ident, Box<Ast>),
//     App(Box<Ast>, Box<Ast>),
// }

// struct Parser<I: Iterator<Item = Token>> {
//     tokens: Peekable<I>,
// }

// impl<I: Iterator<Item = Token>> Parser<I> {
//     fn next(&mut self) -> Result<Option<Ast>> {
//         while let Some(token) = self.tokens.peek() {
//             match token {
//                 Token::Ident(_) => unimplemented!(),
//                 Token::LParen => unimplemented!(),
//                 Token::RParen => unimplemented!(),
//                 Token::BSlash => unimplemented!(),
//                 Token::Dot => unimplemented!(),
//             }
//         }

//         Ok(Some(Ast::Var(Ident("foo".to_string()))))
//     }
// }

// fn parser<I: IntoIterator<Item = Token>>(input: I) -> Result<Vec<Ast>> {
//     let mut parser = Parser {
//         tokens: input.into_iter().peekable(),
//     };
//     let mut parsed = Vec::new();

//     while let Some(term) = parser.next()? {
//         parsed.push(term)
//     }

//     Ok(parsed)
// }

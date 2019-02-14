use std::{iter::Peekable, result::Result as StdResult, str::CharIndices};

fn is_delimiter(c: char) -> bool {
    match c {
        '(' | ')' | ';' | '"' | '\'' | '`' | '|' | '[' | ']' | '{' | '}' => true,
        _ => false,
    }
}

type Result<T, E> = StdResult<T, E>;

#[derive(Debug, PartialEq)]
pub enum LexerError {
    InvalidString(String),
}

#[derive(Debug, PartialEq)]
pub enum Token<'input> {
    // Data
    Identifier(&'input str),
    StringLiteral(&'input str),

    // Symbols
    LBracket,
    RBracket,
    LCurlyBrace,
    RCurlyBrace,
    LParen,
    RParen,
}

pub struct Tokenizer<'input> {
    input: &'input str,
    chars: Peekable<CharIndices<'input>>,
}

impl<'input> Tokenizer<'input> {
    pub fn new(input: &'input str) -> Self {
        Tokenizer {
            input,
            chars: input.char_indices().peekable(),
        }
    }

    fn read_while<F>(&mut self, start: usize, mut proceed: F) -> &'input str
    where
        F: FnMut(char) -> bool,
    {
        while let Some((end, c)) = self.chars.peek().cloned() {
            if proceed(c) {
                self.chars.next();
            } else {
                return &self.input[start..end];
            }
        }

        &self.input[start..self.input.len()]
    }

    fn consume_identifier(&mut self, start: usize) -> Result<Token<'input>, LexerError> {
        let word = self.read_while(start, |c| !is_delimiter(c) && !c.is_whitespace());
        Ok(Token::Identifier(word))
    }

    fn consume_string(&mut self, start: usize) -> Result<Token<'input>, LexerError> {
        let slice = self.read_while(start + 1, |c| c != '"' && c != '\n');

        if slice.len() != 1 {
            if let Some((_, peek)) = self.chars.peek().cloned() {
                return match peek {
                    '"' => {
                        self.chars.next();
                        Ok(Token::StringLiteral(slice))
                    }
                    _ => Err(LexerError::InvalidString(slice.to_string())),
                };
            }
        }

        Err(LexerError::InvalidString(slice.to_string()))
    }

    fn consume_line_comment(&mut self, start: usize) {
        self.read_while(start, |c| c != '\n');
    }
}

impl<'input> Iterator for Tokenizer<'input> {
    type Item = Result<Token<'input>, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((i, c)) = self.chars.next() {
            return Some(match c {
                // Delimiters
                '[' => Ok(Token::LBracket),
                ']' => Ok(Token::RBracket),
                '{' => Ok(Token::LCurlyBrace),
                '}' => Ok(Token::RCurlyBrace),
                '(' => Ok(Token::LParen),
                ')' => Ok(Token::RParen),

                // Literals
                '"' => self.consume_string(i),

                // Trivia
                ';' => {
                    self.consume_line_comment(i);
                    continue;
                }
                c if c.is_whitespace() => continue,

                _ => self.consume_identifier(i),
            });
        }

        None
    }
}

pub fn tokenize(input: &str) -> Vec<Result<Token<'_>, LexerError>> {
    Tokenizer::new(input).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trivia() {
        assert_eq!(tokenize("; this is a line comment"), vec![]);
        assert_eq!(
            tokenize("; this is a line comment\n \t; this is another line comment"),
            vec![]
        )
    }

    #[test]
    fn valid() {
        assert_eq!(
            tokenize(r#"(list"Hi"name(+1-2 3))-!"#),
            &[
                Ok(Token::LParen),
                Ok(Token::Identifier("list")),
                Ok(Token::StringLiteral("Hi")),
                Ok(Token::Identifier("name")),
                Ok(Token::LParen),
                Ok(Token::Identifier("+1-2")),
                Ok(Token::Identifier("3")),
                Ok(Token::RParen),
                Ok(Token::RParen),
                Ok(Token::Identifier("-!"))
            ]
        )
    }

    #[test]
    fn invalid() {
        assert_eq!(
            tokenize("\"Hi! \n"),
            &[Err(LexerError::InvalidString("Hi! ".to_string()))]
        )
    }
}

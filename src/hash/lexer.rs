use std::{char, iter::Peekable, str::Chars};

use super::tokens::{Token, TokenType};

pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a String) -> Self {
        Self {
            source: source.chars().peekable(),
        }
    }

    fn next_char(&mut self) -> char {
        self.source.next().unwrap_or_default()
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.source.peek()
    }

    pub fn lex(&mut self) -> Token {
        match self.source.peek() {
            Some(c) => {
                if c.is_whitespace() {
                    Token::new(self.next_char().to_string(), TokenType::Whitespace)
                } else if c.is_alphabetic() {
                    self.collect_string()
                } else if c.is_numeric() {
                    self.collect_number()
                } else {
                    Token::new(self.next_char().to_string(), TokenType::Unknown)
                }
            }
            None => Token::new("\0".to_string(), TokenType::EOF),
        }
    }

    pub fn peek(&mut self) -> Token {
        let mut peek_lexer = Lexer {
            source: self.source.clone(),
        };

        peek_lexer.lex()
    }

    fn collect_string(&mut self) -> Token {
        let mut buffer = String::new();
        loop {
            buffer.push(self.next_char());
            match self.peek_char() {
                Some(c) => {
                    if !c.is_alphanumeric() {
                        break;
                    }
                }
                None => break,
            }
        }
        Token::new(buffer, TokenType::String)
    }

    fn collect_number(&mut self) -> Token {
        let mut buffer = String::new();
        loop {
            buffer.push(self.next_char());
            match self.peek_char() {
                Some(c) => {
                    if !c.is_alphanumeric() {
                        break;
                    }
                }
                None => break,
            }
        }
        Token::new(buffer, TokenType::Integer)
    }
}

#[cfg(test)]
mod test {}

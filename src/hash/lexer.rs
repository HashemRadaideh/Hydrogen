use std::{char, iter::Peekable, str::Chars};

use super::tokens::{Position, Token};

pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>,
    position: Position,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a String) -> Self {
        Self {
            source: source.chars().peekable(),
            position: Position { col: 1, row: 1 },
        }
    }

    fn next_char(&mut self) -> (Position, char) {
        let current = self.source.next().unwrap_or_default();
        let position = self.position.clone();
        self.position.col += 1;
        if current == '\n' {
            self.position.row += 1;
            self.position.col = 1;
        }
        (position, current)
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.source.peek()
    }

    pub fn peek(&mut self) -> Token {
        let mut peek_lexer = Lexer {
            source: self.source.clone(),
            position: self.position.clone(),
        };

        peek_lexer.lex()
    }

    pub fn lex(&mut self) -> Token {
        match self.source.peek() {
            Some(c) => {
                if c.is_whitespace() {
                    self.consume_whitespace()
                } else if c.is_alphabetic() {
                    self.collect_id()
                } else if c == &'"' {
                    self.collect_string()
                } else if c.is_numeric() {
                    self.collect_number()
                } else {
                    let (position, current) = self.next_char();
                    Token::Unknown(position, current.to_string())
                }
            }
            None => Token::EOF(self.position.clone()),
        }
    }

    fn consume_whitespace(&mut self) -> Token {
        while let Some(&c) = self.peek_char() {
            if !c.is_whitespace() {
                break;
            }
            self.next_char();
        }
        Token::Whitespace
    }

    fn collect<F>(&mut self, condition: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut buffer = String::new();
        while let Some(&c) = self.peek_char() {
            if condition(c) {
                buffer.push(self.next_char().1);
            } else {
                break;
            }
        }
        buffer
    }

    fn collect_id(&mut self) -> Token {
        let buffer = self.collect(|c| c.is_alphanumeric());
        let mut current = self.position.clone();
        current.col -= buffer.len();

        match buffer.as_str() {
            "true" => Token::Boolean(current, true),
            "false" => Token::Boolean(current, false),
            _ => Token::Identifier(current, buffer),
        }
    }

    fn collect_number(&mut self) -> Token {
        let buffer = self.collect(|c| c.is_numeric());
        let mut current = self.position.clone();
        current.col -= buffer.len();
        Token::Integer(current, buffer.parse::<i32>().unwrap())
    }

    fn collect_string(&mut self) -> Token {
        let (current, _) = self.next_char();

        let buffer = self.collect(|c| c != '"');

        // Check if we reached the end of the string or if there is no closing double quote
        if let Some(&c) = self.peek_char() {
            if c == '"' {
                // Consume the closing double quote
                self.next_char();
                Token::String(current.clone(), buffer)
            } else {
                // If there is no closing double quote, return an Unknown token
                Token::Unknown(current.clone(), buffer)
            }
        } else {
            // If there are no more characters, return an Unknown token
            Token::Unknown(self.position.clone(), buffer)
        }
    }
}

#[cfg(test)]
mod test {}

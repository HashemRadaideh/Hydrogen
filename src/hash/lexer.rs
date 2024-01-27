use std::char;

use super::tokens::{Token, TokenType};

pub struct Lexer {
    file: String,
    current: char,
    index: usize,
}

impl Lexer {
    pub fn new(file: &String) -> Self {
        Self {
            file: file.to_owned(),
            current: file.chars().nth(0).unwrap(),
            index: 0,
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        while self.current != '\0' {
            let token = if self.current.is_alphabetic() {
                self.collect_string()
            } else if self.current.is_numeric() {
                self.collect_number()
            } else if self.current.is_whitespace() {
                let token = Token::new(format!("{}", self.current), TokenType::Whitespace);
                self.next();
                token
            } else {
                let token = Token::new(format!("{}", self.current), TokenType::Unknown);
                self.next();
                token
            };

            match token.token {
                TokenType::Whitespace => continue,
                _ => {
                    tokens.push(token);
                }
            }
        }
        tokens.push(Token::new("\0".to_string(), TokenType::EOF));

        return tokens;
    }

    fn next(&mut self) {
        self.index += 1;
        self.current = self.file.chars().nth(self.index).unwrap_or_default();
    }

    fn collect_string(&mut self) -> Token {
        let mut buffer = String::new();
        while self.current.is_alphanumeric() {
            buffer.push(self.current);
            self.next();
        }
        Token::new(buffer, TokenType::String)
    }

    fn collect_number(&mut self) -> Token {
        let mut buffer = String::new();
        while self.current.is_alphanumeric() {
            buffer.push(self.current);
            self.next();
        }
        Token::new(buffer, TokenType::Integear)
    }
}

#[cfg(test)]
mod test {}

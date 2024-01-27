use std::{char, iter::Peekable, str::Chars};

use super::tokens::{Position, Token};

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>,
    position: Position,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
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
        match self.peek_char() {
            Some(&c) => {
                if c.is_whitespace() {
                    self.consume_whitespace()
                } else if c.is_alphabetic() {
                    self.collect_id()
                } else if c == '"' {
                    self.collect_string()
                } else if c.is_numeric() {
                    self.collect_number()
                } else {
                    let (position, current) = self.next_char();
                    match current {
                        '(' => Token::LeftParenthesis(position),
                        ')' => Token::RightParenthesis(position),
                        '{' => Token::LeftBrace(position),
                        '}' => Token::RightBrace(position),
                        '[' => Token::LeftBracket(position),
                        ']' => Token::RightBracket(position),
                        '?' => Token::QuestionMark(position),
                        '$' => Token::DollarSign(position),
                        '#' => Token::Hash(position),
                        ':' => Token::Colon(position),
                        '.' => Token::Point(position),
                        '@' => Token::At(position),
                        '^' => match self.peek_char() {
                            Some(&c) => {
                                if c == '=' {
                                    self.next_char();
                                    Token::CaretEqual(position)
                                } else {
                                    Token::Caret(position)
                                }
                            }
                            None => Token::Caret(position),
                        },
                        '%' => match self.peek_char() {
                            Some(&c) => {
                                if c == '=' {
                                    self.next_char();
                                    Token::PercentEqual(position)
                                } else {
                                    Token::Percent(position)
                                }
                            }
                            None => Token::Percent(position),
                        },
                        '+' => match self.peek_char() {
                            Some(&c) => {
                                if c == '=' {
                                    self.next_char();
                                    Token::PlusEqual(position)
                                } else {
                                    Token::Plus(position)
                                }
                            }
                            None => Token::Plus(position),
                        },
                        '-' => match self.peek_char() {
                            Some(&c) => {
                                if c == '=' {
                                    self.next_char();
                                    Token::MinusEqual(position)
                                } else {
                                    Token::Minus(position)
                                }
                            }
                            None => Token::Minus(position),
                        },
                        '*' => match self.peek_char() {
                            Some(&c) => {
                                if c == '=' {
                                    self.next_char();
                                    Token::AsteriskEqual(position)
                                } else {
                                    Token::Asterisk(position)
                                }
                            }
                            None => Token::Asterisk(position),
                        },
                        '/' => match self.peek_char() {
                            Some(&c) => {
                                if c == '/' {
                                    self.consume_comment()
                                } else if c == '*' {
                                    self.consume_multiline_comment()
                                } else if c == '=' {
                                    self.next_char();
                                    Token::SlashEqual(position)
                                } else {
                                    Token::Slash(position)
                                }
                            }
                            None => Token::Slash(position),
                        },
                        '=' => match self.peek_char() {
                            Some(&c) => {
                                if c == '=' {
                                    self.next_char();
                                    Token::Equals(position)
                                } else {
                                    Token::Equal(position)
                                }
                            }
                            None => Token::Equal(position),
                        },
                        '!' => match self.peek_char() {
                            Some(&c) => {
                                if c == '=' {
                                    self.next_char();
                                    Token::NotEqual(position)
                                } else {
                                    Token::ExplinationMark(position)
                                }
                            }
                            None => Token::ExplinationMark(position),
                        },
                        '>' => match self.peek_char() {
                            Some(&c) => {
                                if c == '=' {
                                    self.next_char();
                                    Token::GreaterThanOrEqual(position)
                                } else {
                                    Token::GreaterThan(position)
                                }
                            }
                            None => Token::GreaterThan(position),
                        },
                        '<' => match self.peek_char() {
                            Some(&c) => {
                                if c == '=' {
                                    self.next_char();
                                    Token::LessThanOrEqual(position)
                                } else {
                                    Token::LessThan(position)
                                }
                            }
                            None => Token::LessThan(position),
                        },
                        '&' => match self.peek_char() {
                            Some(&c) => {
                                if c == '&' {
                                    self.next_char();
                                    Token::And(position)
                                } else {
                                    Token::Ampersand(position)
                                }
                            }
                            None => Token::Ampersand(position),
                        },
                        '|' => match self.peek_char() {
                            Some(&c) => {
                                if c == '|' {
                                    self.next_char();
                                    Token::Or(position)
                                } else {
                                    Token::Unknown(position, current.to_string())
                                }
                            }
                            None => Token::Unknown(position, current.to_string()),
                        },
                        _ => Token::Unknown(position, current.to_string()),
                    }
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
        self.lex()
    }

    fn consume_comment(&mut self) -> Token {
        while let Some(&c) = self.peek_char() {
            if c == '\n' {
                break;
            }
            self.next_char();
        }
        self.lex()
    }

    fn consume_multiline_comment(&mut self) -> Token {
        while let Some(&c) = self.peek_char() {
            if c == '*' {
                self.next_char();
                if let Some(&c) = self.peek_char() {
                    if c == '/' {
                        self.next_char();
                        break;
                    }
                }
            }
            self.next_char();
        }
        self.lex()
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
            "if" => Token::Keyword(current, buffer),
            "else" => Token::Keyword(current, buffer),
            "for" => Token::Keyword(current, buffer),
            "in" => Token::Keyword(current, buffer),
            "while" => Token::Keyword(current, buffer),
            "break" => Token::Keyword(current, buffer),
            "continue" => Token::Keyword(current, buffer),
            "number" => Token::Type(current, buffer),
            "string" => Token::Type(current, buffer),
            "true" => Token::Boolean(current, buffer),
            "false" => Token::Boolean(current, buffer),
            _ => Token::Identifier(current, buffer),
        }
    }

    fn collect_number(&mut self) -> Token {
        let buffer = self.collect(|c| c.is_numeric());
        let mut current = self.position.clone();
        current.col -= buffer.len();
        Token::Number(current, buffer)
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

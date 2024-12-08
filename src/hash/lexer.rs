use std::{char, iter::Peekable, str::Chars};

use super::tokens::{Position, Token};

/// Lexer struct responsible for tokenizing the source code.
/// # TODO:
/// - [x] tokenize identifiers
/// - [x] tokenize numbers
/// - [x] tokenize strings
/// - [x] tokenize operators
/// - [ ] fix the number tokinizing to parse multiple formats of numbers
/// - [ ] fix the string tokinizing to parse escaped characters
/// - [ ] MAKE A ZERO COPY parser stop using String and use &str
///
/// # Example of number formats
/// ```
/// 1234        // integer
/// 3.14159     // float
/// 3E2         // scientific notation
/// 314E-2      // scientific notation
/// 3.14E2      // scientific notation with a floating point
/// 3_141_592   // with separators
/// 3_14_15_92  // other supported format
/// ```
#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>, // Peekable iterator over characters in the source code
    position: Position,          // Current position in the source code
}

impl<'a> Lexer<'a> {
    /// Creates a new Lexer instance from the given source code.
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.chars().peekable(),
            position: Position { col: 1, row: 1 },
        }
    }

    /// Retrieves the next character from the source code and updates the position.
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

    /// Peeks at the next character in the source code without consuming it.
    fn peek_char(&mut self) -> Option<&char> {
        self.source.peek()
    }

    /// Peeks at the next token without consuming it.
    pub fn peek(&mut self) -> Token {
        let mut peek_lexer = Lexer {
            source: self.source.clone(),
            position: self.position.clone(),
        };

        peek_lexer.lex()
    }

    /// Lexes and returns the next token from the source code.
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

                        ',' => Token::Comma(position),

                        '?' => Token::QuestionMark(position),

                        '$' => Token::DollarSign(position),

                        '#' => Token::Hash(position),

                        ':' => Token::Colon(position),

                        '.' => Token::Dot(position),

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
            None => Token::Eof(self.position.clone()),
        }
    }

    /// Consumes whitespace characters until a non-whitespace character is encountered.
    fn consume_whitespace(&mut self) -> Token {
        while let Some(&c) = self.peek_char() {
            if !c.is_whitespace() {
                break;
            }
            self.next_char();
        }
        self.lex()
    }

    /// Consumes characters until a newline character is encountered, indicating the end of a line comment.
    fn consume_comment(&mut self) -> Token {
        while let Some(&c) = self.peek_char() {
            if c == '\n' {
                break;
            }
            self.next_char();
        }
        self.lex()
    }

    /// Consumes characters until the closing delimiter of a multiline comment is encountered.
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

    /// Collects characters that satisfy the provided condition until a character that does not satisfy the condition is encountered.
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

    /// Collects characters to form an identifier or a keyword.
    fn collect_id(&mut self) -> Token {
        let buffer = self.collect(|c| c.is_alphanumeric());
        let mut current = self.position.clone();
        current.col -= buffer.len();

        match buffer.as_str() {
            "if" => Token::Keyword(current, buffer),
            "else" => Token::Keyword(current, buffer),
            "while" => Token::Keyword(current, buffer),
            "break" => Token::Keyword(current, buffer),
            "continue" => Token::Keyword(current, buffer),
            "in" => Token::In(current),
            "as" => Token::As(current),
            "num" => Token::Type(current, buffer),
            "str" => Token::Type(current, buffer),
            "bool" => Token::Type(current, buffer),
            "true" => Token::Boolean(current, buffer),
            "false" => Token::Boolean(current, buffer),
            _ => Token::Identifier(current, buffer),
        }
    }

    /// Collects characters to form a numeric literal.
    fn collect_number(&mut self) -> Token {
        let buffer = self.collect(|c| c.is_numeric());
        let mut current = self.position.clone();
        current.col -= buffer.len();
        Token::Number(current, buffer)
    }

    /// Collects characters to form a string literal.
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
mod test {
    use super::*;

    #[test]
    fn test_lexer() {
        let program = r#"
            hi() {
                print()
            }

            main() {
                hello(): num {
                    var1 = 1234
                    var2 = 1234
                }

                hello()

                var1: num = 1234
                var2 = var1 + 1234

                var3: num = lambda() {
                    var: str = "Hello, World!"
                }

                var4: bool = true
            }
        "#;

        let mut lexer = Lexer::new(program);
        let mut tokens = Vec::new();

        loop {
            let token = lexer.lex();
            if let Token::Eof(_) = token {
                break;
            }
            tokens.push(token);
        }

        // TODO: Add more specific assertions based on the expected tokens
        // For example, you can assert the types and positions of tokens.
    }
}

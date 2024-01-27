use super::{
    ast::{Error, Node},
    lexer::Lexer,
    tokens::Token,
};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(program: &'a String) -> Self {
        Self {
            lexer: Lexer::new(&program),
        }
    }

    fn next(&mut self) -> Token {
        self.lexer.lex()
    }

    fn peek(&mut self) -> Token {
        self.lexer.peek()
    }

    pub fn parse(&mut self) -> Result<Vec<Box<Node>>, Vec<Box<Error>>> {
        let statements = Vec::new();
        let errors = Vec::new();

        loop {
            let current = self.next();
            if !matches!(current, Token::Whitespace) {
                println!("{:?}", current);
            }
            match current {
                Token::LeftParenthesis(_) => {}
                Token::RightParenthesis(_) => {}
                Token::LeftBrace(_) => {}
                Token::RightBrace(_) => {}
                Token::LeftBracket(_) => {}
                Token::RightBracket(_) => {}
                Token::Assignment(_) => {}
                Token::Plus(_) => {}
                Token::Minus(_) => {}
                Token::Asterisk(_) => {}
                Token::Slash(_) => {}
                Token::Ampersand(_) => {}
                Token::DollarSign(_) => {}
                Token::Hash(_) => {}
                Token::ExplinationMark(_) => {}
                Token::QuestionMark(_) => {}
                Token::EscapeCode(_, _) => {}
                Token::Identifier(_, _) => {}
                Token::String(_, _) => {}
                Token::Boolean(_, _) => {}
                Token::Integer(_, _) => {}
                Token::Float(_, _) => {}
                Token::Keyword(_, _) => {}
                Token::Unknown(_, _) => {}
                Token::Whitespace => {}
                Token::EOF(_) => break,
            }
        }

        if errors.is_empty() {
            Ok(statements)
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod test {}

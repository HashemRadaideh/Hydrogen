use super::{
    ast::{ASTNode, ASTree},
    lexer::Lexer,
    tokens::{Token, TokenType},
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

    pub fn parse(&mut self) -> Result<ASTree, Vec<Box<ParseError>>> {
        let mut statements = Vec::new();
        let mut errors = Vec::new();

        loop {
            match self.peek().token {
                TokenType::EOF => {
                    break;
                }
                _ => {
                    let statement = self.parse_statement();
                    match statement {
                        Ok(stmt) => statements.push(Box::new(stmt)),
                        Err(err) => errors.push(Box::new(err)),
                    }
                }
            }
        }

        if errors.len() > 0 {
            Err(errors)
        } else {
            let mut ast = ASTree::new();
            ast.root = Some(Box::new(ASTNode::Program(statements)));
            Ok(ast)
        }
    }

    fn parse_statement(&mut self) -> Result<ASTNode, ParseError> {
        match self.peek().token {
            TokenType::String => Ok(ASTNode::Identifier(self.next().value.clone())),
            TokenType::Integer => Ok(ASTNode::Identifier(self.next().value.clone())),
            TokenType::Whitespace => Ok(ASTNode::Identifier(self.next().value.clone())),
            TokenType::Unknown => Err(ParseError::UnknownToken),
            TokenType::EOF => Ok(ASTNode::Identifier(self.next().value.clone())),
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    UnknownToken,
    // Add more error types as needed
}

#[cfg(test)]
mod test {}

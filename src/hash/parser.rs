use super::{
    ast::{ASTNode, ASTree},
    tokens::{Token, TokenType},
};

pub struct Parser {
    lexed: Vec<Token>,
    current: Token,
    index: usize,
}

impl Parser {
    pub fn new(lexed: Vec<Token>) -> Self {
        Self {
            lexed: lexed.clone(),
            current: lexed[0].clone(),
            index: 0,
        }
    }

    pub fn parse(&mut self) -> ASTree {
        let mut ast = ASTree::new();
        let program_node = self.parse_program();
        ast.root = Some(Box::new(program_node));
        ast
    }

    fn parse_program(&mut self) -> ASTNode {
        let mut statements = Vec::new();

        loop {
            match self.current.token {
                TokenType::EOF => {
                    break;
                }
                _ => {
                    let statement = self.parse_statement();
                    statements.push(Box::new(statement));
                }
            }
        }

        ASTNode::Program(statements)
    }

    fn parse_statement(&mut self) -> ASTNode {
        match self.current.token {
            TokenType::String
            | TokenType::Integear
            | TokenType::Whitespace
            | TokenType::Unknown => {
                let identifier = self.current.value.clone();
                self.next();
                ASTNode::Identifier(identifier)
            }
            _ => {
                let identifier = self.current.value.clone();
                self.expect(TokenType::String);
                self.expect(TokenType::Unknown);
                let expression = self.parse_statement();
                ASTNode::VariableDeclaration(identifier, Box::new(expression))
            }
        }
    }

    fn expect(&mut self, expected_type: TokenType) {
        match self.current.token {
            TokenType::String => {
                self.next();
            }
            TokenType::Integear => {
                self.next();
            }
            TokenType::Unknown => {
                self.next();
            }
            TokenType::Whitespace => {
                self.next();
            }
            TokenType::EOF => {
                self.next();
            }
            _ => {
                panic!(
                    "Expected token of type {:?}, but found {:?}",
                    expected_type, self.current
                );
            }
        }
    }

    fn next(&mut self) {
        if self.index < self.lexed.len() - 1 {
            self.index += 1;
            self.current = self.lexed[self.index].clone();
        } else {
            self.current = Token::new(String::new(), TokenType::EOF);
        }
    }
}

#[cfg(test)]
mod test {}

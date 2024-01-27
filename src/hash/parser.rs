use super::{
    ast::{Error, Node},
    lexer::Lexer,
    tokens::Token,
};

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(program: &'a str) -> Self {
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
        let mut program = Vec::new();
        let mut errors = Vec::new();

        loop {
            // println!("{:?}", self.peek());
            match self.peek() {
                Token::Unknown(_, _) => {
                    let token = self.next();
                    errors.push(Box::new(Error::UnknownToken(token)));
                }
                Token::EOF(_) => break,
                _ => match self.parse_node() {
                    Ok(node) => {
                        program.push(node);
                    }
                    Err(error) => {
                        errors.push(error);
                    }
                },
            }
        }

        if errors.is_empty() {
            Ok(program.clone())
        } else {
            Err(errors.clone())
        }
    }

    fn parse_node(&mut self) -> Result<Box<Node>, Box<Error>> {
        let token = self.next();
        match token.clone() {
            Token::RightParenthesis(_) | Token::RightBrace(_) | Token::RightBracket(_) => {
                Ok(Box::new(Node::End))
            }
            Token::String(_, string) => Ok(Box::new(Node::StringLiteral(string))),
            Token::Number(_, number) => Ok(Box::new(Node::NumberLiteral(number))),
            Token::Plus(_) | Token::Minus(_) => {
                let expression = self.parse_expression()?;
                Ok(Box::new(Node::UnaryExpression(
                    Box::new(Node::Operator(token.to_string())),
                    expression,
                )))
            }
            Token::Identifier(_, id) => match self.peek() {
                Token::LeftParenthesis(_) => {
                    if let Ok(value) = self.parse_function_definition() {
                        Ok(Box::new(Node::FunctionDefinition(
                            Box::new(Node::Identifier(id)),
                            value[0].clone(),
                            value[1].clone(),
                        )))
                    } else {
                        Err(Box::new(Error::UnexpectedToken(token.clone())))
                    }
                }
                Token::Equal(_) => {
                    if let Ok(value) = self.parse_variable_definition() {
                        Ok(Box::new(Node::VariableDefinition(
                            Box::new(Node::Identifier(id)),
                            value.clone(),
                        )))
                    } else {
                        Err(Box::new(Error::UnexpectedToken(token.clone())))
                    }
                }
                _ => Err(Box::new(Error::UnexpectedToken(token))),
            },
            _ => Err(Box::new(Error::UnexpectedToken(token))),
        }
    }

    fn parse_function_definition(&mut self) -> Result<Vec<Box<Node>>, Box<Error>> {
        let param = self.parse_parameters()?;
        let body = self.parse_block()?;
        Ok(vec![param, body])
    }

    fn parse_parameters(&mut self) -> Result<Box<Node>, Box<Error>> {
        let mut parameters = Vec::new();
        let mut errors = Vec::new();

        loop {
            match self.next() {
                Token::RightParenthesis(_) => break,
                // _ => continue,
                _ => match self.parse_node() {
                    Ok(parameter) => match *parameter {
                        Node::End => break,
                        _ => parameters.push(parameter),
                    },
                    Err(error) => {
                        errors.push(error);
                    }
                },
            }
        }

        if errors.is_empty() {
            Ok(Box::new(Node::Parameters(parameters)))
        } else {
            Err(Box::new(Error::Errors(errors)))
        }
    }

    fn parse_block(&mut self) -> Result<Box<Node>, Box<Error>> {
        let mut statements = Vec::new();
        let mut errors = Vec::new();

        loop {
            match self.next() {
                Token::RightBrace(_) => break,
                // _ => continue,
                _ => match self.parse_node() {
                    Ok(statement) => match *statement {
                        Node::End => break,
                        _ => statements.push(statement),
                    },
                    Err(error) => {
                        errors.push(error);
                    }
                },
            }
        }

        if errors.is_empty() {
            Ok(Box::new(Node::Block(statements)))
        } else {
            Err(Box::new(Error::Errors(errors)))
        }
    }

    fn parse_variable_definition(&mut self) -> Result<Box<Node>, Box<Error>> {
        self.next();
        let expression = self.parse_expression()?;
        Ok(expression)
    }

    fn parse_expression(&mut self) -> Result<Box<Node>, Box<Error>> {
        let mut left = self.parse_term()?;

        while let Some(op) = self.match_binary_operator() {
            let right = self.parse_term()?;
            left = Box::new(Node::BinaryExpression(
                left,
                Box::new(Node::Operator(op)),
                right,
            ));
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Box<Node>, Box<Error>> {
        let mut left = self.parse_factor()?;

        while let Some(op) = self.match_binary_operator() {
            let right = self.parse_factor()?;
            left = Box::new(Node::BinaryExpression(
                left,
                Box::new(Node::Operator(op)),
                right,
            ));
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Box<Node>, Box<Error>> {
        if let Some(op) = self.match_unary_operator() {
            let expression = self.parse_factor()?;
            Ok(Box::new(Node::UnaryExpression(
                Box::new(Node::Operator(op)),
                expression,
            )))
        } else {
            self.parse_literal_or_identifier()
        }
    }

    fn match_binary_operator(&mut self) -> Option<String> {
        match self.peek() {
            Token::Plus(_) | Token::Minus(_) | Token::Equal(_) | Token::NotEqual(_) => {
                Some(self.next().to_string())
            }
            _ => None,
        }
    }

    fn match_unary_operator(&mut self) -> Option<String> {
        match self.peek() {
            Token::Plus(_) | Token::Minus(_) => Some(self.next().to_string()),
            _ => None,
        }
    }

    fn parse_literal_or_identifier(&mut self) -> Result<Box<Node>, Box<Error>> {
        let token = self.next();
        match token.clone() {
            Token::String(_, string) => Ok(Box::new(Node::StringLiteral(string))),
            Token::Number(_, number) => Ok(Box::new(Node::NumberLiteral(number))),
            Token::Identifier(_, id) => Ok(Box::new(Node::Identifier(id))),
            _ => Err(Box::new(Error::UnexpectedToken(token))),
        }
    }
}

#[cfg(test)]
mod test {}

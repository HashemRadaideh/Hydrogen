use super::{
    ast::{ASTError, ASTNode, Error, Errors, Node, Nodes},
    lexer::Lexer,
    tokens::Token,
};

/// Parser Generates an abstract syntax tree from a program source code
///
/// # Eamples
/// ```
/// let mut parser = Parser::new("1 + 2");
/// let ast = parser.parse();
///
/// assert!(ast.is_ok());
/// ```
///
/// # Hydrogen example
/// ```hy
/// hi() {
///   print()
/// }
///
/// main() {
///   hello(): num {
///       var1 = 1234
///       var2 = 1234
///   }
///
///   hello()
///
///   var1: num = 1234
///   var2 = var1 + 1234
///
///   var3: num = lambda() {
///       var: str = "Hello, World!"
///   }
///
///   var4: bool = true
/// }
/// ```
///
/// # TODO:
/// - [x] implement parsing for statements
/// - [x] implement parsing for functions
/// - [x] implement parsing for blocks
/// - [x] implement parsing for variables
/// - [x] implement parsing for expressions
/// - [ ] implement parsing for parameneters
/// - [ ] implement parsing for arguments
/// - [ ] implement parsing for keywords
/// - [ ] fix the parsing of functions to include =
/// - [ ] change the parsing of function body to
///       parce_node instead of parse_block
/// - [ ] change variable declaration's expression to be optional
/// - [ ] fix the parser's error propagation
/// - [ ] clean the api of the parser
/// - [ ] add user defined types
/// - [ ] convert the parser into a ZERO COPY
///
/// # References to zero copy parser
/// https://itnext.io/rust-the-joy-of-safe-zero-copy-parsers-8c8581db8ab2
/// https://www.reddit.com/r/rust/comments/5msqlh/explainingillustrating_zerocopy/
/// https://swatinem.de/blog/magic-zerocopy/
/// https://www.roxlu.com/2015/052/building-a-zero-copy-parser
#[derive(Debug, Clone)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    /// Creates a new Parser instance with the given program source code.
    pub fn new(program: &'a str) -> Self {
        Self {
            lexer: Lexer::new(&program),
        }
    }

    /// Advances the lexer and returns the next token.
    fn next(&mut self) -> Token {
        self.lexer.lex()
    }

    /// Peeks at the next token without advancing the lexer.
    fn peek(&mut self) -> Token {
        self.lexer.peek()
    }

    /// Parses the entire program and returns the abstract syntax tree.
    pub fn parse(&mut self) -> Result<Nodes, Errors> {
        let mut program = Vec::new();
        let mut errors = Vec::new();

        loop {
            match self.peek() {
                Token::Unknown(_, _) => {
                    let token = self.next();
                    errors.push(Box::new(ASTError::UnknownToken(token)));
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

    fn parse_node(&mut self) -> Result<Node, Error> {
        let token = self.next();
        match token.clone() {
            Token::RightParenthesis(_) | Token::RightBrace(_) | Token::RightBracket(_) => {
                Ok(Box::new(ASTNode::Delimiter))
            }
            Token::String(_, string) => Ok(Box::new(ASTNode::StringLiteral(string))),
            Token::Number(_, number) => Ok(Box::new(ASTNode::NumberLiteral(number))),
            Token::Boolean(_, boolean) => {
                Ok(Box::new(ASTNode::BooleanLiteral(if boolean == "true" {
                    true
                } else {
                    false
                })))
            }
            Token::Type(_, t) => {
                if t == "num" {
                    Ok(Box::new(ASTNode::NumberType))
                } else if t == "str" {
                    Ok(Box::new(ASTNode::StringType))
                } else if t == "bool" {
                    Ok(Box::new(ASTNode::BooleanType))
                } else {
                    Err(Box::new(ASTError::UnknownToken(token)))
                }
            }
            Token::Plus(_) | Token::Minus(_) => {
                let expression = self.parse_expression()?;
                Ok(Box::new(ASTNode::UnaryExpression(
                    Box::new(ASTNode::Operator(token.to_string())),
                    expression,
                )))
            }
            Token::Identifier(_, id) => match self.peek() {
                Token::LeftParenthesis(_) => {
                    if let Ok(value) = self.parse_function() {
                        if value.len() == 1 {
                            Ok(Box::new(ASTNode::FunctionCall(
                                Box::new(ASTNode::Identifier(id)),
                                value[0].clone(),
                            )))
                        } else {
                            Ok(Box::new(ASTNode::FunctionDefinition(
                                Box::new(ASTNode::Identifier(id)),
                                value[0].clone(),
                                value[1].clone(),
                                value[2].clone(),
                            )))
                        }
                    } else {
                        Err(Box::new(ASTError::UnexpectedToken(token.clone())))
                    }
                }
                Token::Colon(_) | Token::Equal(_) => {
                    if let Ok(value) = self.parse_variable() {
                        Ok(Box::new(ASTNode::VariableDefinition(
                            Box::new(ASTNode::Identifier(id)),
                            value[0].clone(),
                            value[1].clone(),
                        )))
                    } else {
                        Err(Box::new(ASTError::UnexpectedToken(token.clone())))
                    }
                }
                _ => Ok(Box::new(ASTNode::Identifier(id))),
            },
            _ => Err(Box::new(ASTError::UnexpectedToken(token))),
        }
    }

    fn parse_function(&mut self) -> Result<Nodes, Error> {
        match self.parse_parameters() {
            Ok(param) => match self.peek() {
                Token::LeftBrace(_) => match self.parse_block() {
                    Ok(body) => Ok(vec![param, Box::new(ASTNode::Return(None)), body]),
                    Err(errors) => Err(errors),
                },
                Token::Colon(_) => match self.parse_return() {
                    Ok(ret) => match self.peek() {
                        Token::LeftBrace(_) => match self.parse_block() {
                            Ok(body) => Ok(vec![param, Box::new(ASTNode::Return(Some(ret))), body]),
                            Err(errors) => Err(errors),
                        },
                        _ => Err(Box::new(ASTError::UnexpectedToken(self.next()))),
                    },
                    Err(errors) => Err(errors),
                },
                _ => Ok(vec![param]),
            },
            Err(error) => Err(error),
        }
    }

    fn parse_return(&mut self) -> Result<Node, Error> {
        self.next();
        let ret = self.parse_node()?;

        Ok(ret)
    }

    fn parse_parameters(&mut self) -> Result<Node, Error> {
        let mut parameters = Vec::new();
        let mut errors = Vec::new();

        self.next();
        loop {
            match self.peek() {
                Token::RightParenthesis(_) => {
                    self.next();
                    break;
                }
                _ => match self.parse_node() {
                    Ok(parameter) => match *parameter {
                        ASTNode::Delimiter => break,
                        _ => parameters.push(parameter),
                    },
                    Err(error) => {
                        errors.push(error);
                    }
                },
            }
        }

        if errors.is_empty() {
            match self.peek() {
                Token::LeftBrace(_) => Ok(Box::new(ASTNode::Parameters(parameters))),
                _ => Ok(Box::new(ASTNode::Arguments(parameters))),
            }
        } else {
            Err(Box::new(ASTError::Errors(errors)))
        }
    }

    fn parse_block(&mut self) -> Result<Node, Error> {
        let mut statements = Vec::new();
        let mut errors = Vec::new();

        self.next();
        loop {
            match self.peek() {
                Token::RightBrace(_) => {
                    self.next();
                    break;
                }
                _ => match self.parse_node() {
                    Ok(statement) => match *statement {
                        ASTNode::Delimiter => {
                            self.next();
                            break;
                        }
                        _ => statements.push(statement),
                    },
                    Err(error) => {
                        errors.push(error);
                    }
                },
            }
        }

        if errors.is_empty() {
            Ok(Box::new(ASTNode::Block(statements)))
        } else {
            Err(Box::new(ASTError::Errors(errors)))
        }
    }

    fn parse_variable(&mut self) -> Result<Nodes, Error> {
        let token = self.next();
        match token {
            Token::Equal(_) => {
                let expression = self.parse_expression()?;
                Ok(vec![Box::new(ASTNode::Type(None)), expression])
            }
            Token::Colon(_) => {
                let t = self.parse_type()?;
                self.next();
                let expression = self.parse_expression()?;
                Ok(vec![Box::new(ASTNode::Type(Some(t))), expression])
            }
            _ => Err(Box::new(ASTError::UnknownToken(token))),
        }
    }

    fn parse_type(&mut self) -> Result<Node, Error> {
        let t = self.parse_node()?;

        Ok(t)
    }

    fn parse_expression(&mut self) -> Result<Node, Error> {
        let mut left = self.parse_term()?;

        while let Some(op) = self.match_binary_operator() {
            let right = self.parse_term()?;
            left = Box::new(ASTNode::BinaryExpression(
                left,
                Box::new(ASTNode::Operator(op)),
                right,
            ));
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Node, Error> {
        let mut left = self.parse_factor()?;

        while let Some(op) = self.match_binary_operator() {
            let right = self.parse_factor()?;
            left = Box::new(ASTNode::BinaryExpression(
                left,
                Box::new(ASTNode::Operator(op)),
                right,
            ));
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Node, Error> {
        if let Some(op) = self.match_unary_operator() {
            let expression = self.parse_factor()?;
            Ok(Box::new(ASTNode::UnaryExpression(
                Box::new(ASTNode::Operator(op)),
                expression,
            )))
        } else {
            self.parse_node()
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser() {
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

        let mut parser = Parser::new(program);
        let result = parser.parse();

        assert!(result.is_ok());

        // TODO: Add more specific assertions based on your expected AST structure
        // For example, you can assert the structure of the AST, the types of nodes, etc.
    }
}

use std::fmt;

use super::tokens::Token;

/// Alias for Nodes type
pub type Tree = Nodes;

/// Alias for Boxed ASTNode
pub type Node = Box<ASTNode>;

/// Vector of AST nodes
pub type Nodes = Vec<Node>;

/// Alias for boxed ASTError
pub type Error = Box<ASTError>;

/// Vector of AST errors
pub type Errors = Vec<Error>;

/// Enum representing different types of AST nodes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ASTNode {
    StringType,
    StringLiteral(String),
    BooleanType,
    BooleanLiteral(bool),
    NumberType,
    NumberLiteral(String),
    Identifier(String),
    Operator(String),

    // Placeholder for potential future node types
    // NumberLiteral(Box<Node>),
    // IntegerLiteral(i32),
    // FloatLiteral(f32),
    /// Variable definition: (identifier, type, expression)
    VariableDefinition(Node, Node, Node),

    /// Type: (type)
    Type(Option<Node>),

    /// Unary expression: (operator, expression)
    UnaryExpression(Node, Node),

    /// Binary expression: (expression, operator, expression)
    BinaryExpression(Node, Node, Node),

    /// Function definition: (identifier, parameters, return, body)
    FunctionDefinition(Node, Node, Node, Node),

    /// Parameters: (variable declarations)
    Parameters(Nodes),

    /// Return: (type)
    Return(Option<Node>),

    /// Block: (statements)
    Block(Nodes),

    /// Function call: (identifier, arguments)
    FunctionCall(Node, Node),

    /// Arguments: (variables)
    Arguments(Nodes),

    /// Delimiter end the parsing of the current statement
    Delimiter,
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASTNode::StringLiteral(value) => write!(f, "\"{}\"", value),
            ASTNode::BooleanLiteral(value) => write!(f, "{}", value),
            ASTNode::NumberLiteral(value) => write!(f, "{}", value),
            ASTNode::Identifier(name) => write!(f, "{}", name),
            ASTNode::Operator(op) => write!(f, "{}", op),
            ASTNode::UnaryExpression(op, expr) => write!(f, "({} {})", op, expr),
            ASTNode::BinaryExpression(left, op, right) => write!(f, "({} {} {})", left, op, right),
            ASTNode::VariableDefinition(name, t, expr) => write!(f, "{}: {} = {}", name, t, expr),
            ASTNode::FunctionDefinition(name, params, ret, body) => {
                write!(f, "{}({}): {} {}", name, params, ret, body)
            }
            ASTNode::Parameters(params) => {
                let params_str: Vec<String> = params.iter().map(|p| p.to_string()).collect();
                write!(f, "({})", params_str.join(", "))
            }
            ASTNode::Block(statements) => {
                let statements_str: Vec<String> =
                    statements.iter().map(|s| s.to_string()).collect();
                write!(f, "{{\n{}\n}}", statements_str.join("\n"))
            }
            ASTNode::FunctionCall(name, args) => {
                write!(f, "{}({})", name, args)
            }
            ASTNode::Arguments(args) => {
                let args_str: Vec<String> = args.iter().map(|a| a.to_string()).collect();
                write!(f, "({})", args_str.join(", "))
            }
            ASTNode::Type(t) => match t {
                Some(t) => write!(f, "{}", t),
                None => write!(f, "none"),
            },
            ASTNode::Return(ret) => match ret {
                Some(ret) => write!(f, "{}", ret),
                None => write!(f, "none"),
            },
            ASTNode::StringType => write!(f, "str"),
            ASTNode::BooleanType => write!(f, "bool"),
            ASTNode::NumberType => write!(f, "num"),
            ASTNode::Delimiter => todo!(),
        }
    }
}

/// Enum representing different types of AST errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ASTError {
    UnknownToken(Token),
    UnexpectedToken(Token),
    EarlyEOF(Token),
    Errors(Errors),
}

impl fmt::Display for ASTError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASTError::UnknownToken(error) => write!(f, "ERROR: {}", error),
            ASTError::UnexpectedToken(error) => write!(f, "ERROR: {}", error),
            ASTError::Errors(errors) => write!(f, "ERROR: {:?}", errors),
            ASTError::EarlyEOF(errors) => write!(f, "ERROR: {:?}", errors),
        }
    }
}

#[cfg(test)]
mod tests {}

use std::fmt;

use super::tokens::Token;

#[derive(Debug, Clone)]
pub enum ASTNode {
    StringType,
    StringLiteral(String),
    BooleanType,
    BooleanLiteral(bool),
    NumberType,
    NumberLiteral(String),
    Identifier(String),
    Operator(String),

    // takes either an Integer or Float literals
    // NumberLiteral(Box<Node>),
    // IntegerLiteral(i32),
    // FloatLiteral(f32),

    //                 [identifier, type, expresion]
    VariableDefinition(Node, Node, Node),

    //  [type]
    Type(Option<Node>),

    //             [operator, expression]
    UnaryExpression(Node, Node),

    //              [expression, operator, expression]
    BinaryExpression(Node, Node, Node),

    //                [identifier, parameters, return, body]
    FunctionDefinition(Node, Node, Node, Node),
    //        [variable declarations]
    Parameters(Vec<Node>),
    //  [type]
    Return(Option<Node>),
    //   [statements]
    Block(Vec<Node>),

    //          [identifier, arguments]
    FunctionCall(Node, Node),
    //       [variables]
    Arguments(Vec<Node>),

    //
    End,
}

pub type Node = Box<ASTNode>;

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
            ASTNode::End => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ASTError {
    UnknownToken(Token),
    UnexpectedToken(Token),
    Errors(Vec<Error>),
}

pub type Error = Box<ASTError>;

impl fmt::Display for ASTError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASTError::UnknownToken(error) => write!(f, "ERROR: {}", error),
            ASTError::UnexpectedToken(error) => write!(f, "ERROR: {}", error),
            ASTError::Errors(errors) => write!(f, "ERROR: {:?}", errors),
        }
    }
}

#[cfg(test)]
mod tests {}

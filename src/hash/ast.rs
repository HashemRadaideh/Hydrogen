#![allow(unused)]

use super::tokens::Token;

#[derive(Debug, Clone)]
pub enum Node {
    StringLiteral(String),
    BooleanLiteral(bool),
    NumberLiteral(String),
    Identifier(String),
    Operator(String),

    // takes either an Integer or Float literals
    // NumberLiteral(Box<Node>),
    // IntegerLiteral(i32),
    // FloatLiteral(f32),

    //                [identifier, type,             expression]
    // VariableAssignment(Box<Node>, Option<Box<Node>>, Box<Node>),

    //             [operator, expression]
    UnaryExpression(Box<Node>, Box<Node>),

    //              [expression, operator, expression]
    BinaryExpression(Box<Node>, Box<Node>, Box<Node>),

    //                 [identifier, type]
    VariableDefinition(Box<Node>, Box<Node>),

    //                [identifier, parameters,    body]
    FunctionDefinition(Box<Node>, Box<Node>, Box<Node>),
    //        [variable declarations]
    Parameters(Vec<Box<Node>>),
    //
    End,
    //   [statements]
    Block(Vec<Box<Node>>),

    //          [identifier, arguments]
    FunctionCall(Box<Node>, Vec<Box<Node>>),
    //       [variables]
    Arguments(Vec<Box<Node>>),
}

#[derive(Debug, Clone)]
pub enum Error {
    UnknownToken(Token),
    UnexpectedToken(Token),
    Errors(Vec<Box<Error>>),
}

#[cfg(test)]
mod tests {}

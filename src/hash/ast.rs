use super::tokens::{Position, Token};

#[derive(Debug)]
pub enum Node {
    StringLiteral(String),
    BooleanLiteral(bool),

    // takes either an Integer or Float literals
    NumberLiteral(Box<Node>),
    IntegerLiteral(i32),
    FloatLiteral(f32),

    //                [identifier, type,             expression]
    VariableAssignment(Box<Node>, Option<Box<Node>>, Box<Node>),

    //             [operator, expression]
    UnaryExpression(Box<Node>, Box<Node>),

    //              [expression, operator, expression]
    BinaryExpression(Box<Node>, Box<Node>, Box<Node>),

    //                 [identifier, type]
    VariableDeclaration(Box<Node>, Box<Node>),

    Variable(String),

    Statement(Box<Node>),

    //                [identifier, parameters,    body]
    FunctionDefinition(Box<Node>, Box<Node>, Box<Node>),
    //        [variable declarations]
    Parameters(Vec<Box<Node>>),
    //   [statements]
    Block(Vec<Box<Node>>),

    //          [identifier, arguments]
    FunctionCall(Box<Node>, Vec<Box<Node>>),
    //       [variables]
    Arguments(Vec<Box<Node>>),

    Skip,
    End,
}

#[derive(Debug)]
pub enum Error {
    UnknownToken(Token),
    UnexpectedToken(Token),
}

#[cfg(test)]
mod tests {}

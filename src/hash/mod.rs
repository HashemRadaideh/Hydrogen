use std::io;

use crate::hash::ast::{ASTNode, Node};

use self::{
    ast::{Errors, Tree},
    parser::Parser,
};

use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};

/// Module containing abstract syntax tree (AST) definitions.
pub mod ast;
/// Module containing lexer implementation.
pub mod lexer;
/// Module containing parser implementation.
pub mod parser;
/// Module containing token definitions.
pub mod tokens;

/// Validates the input content and returns the AST or errors.
///
/// # Arguments
///
/// * `content` - The content to be parsed and validated.
///
/// # Returns
///
/// * `Result<Tree, Errors>` - Ok(Tree) if parsing is successful, Err(Errors) if there are errors.
pub fn validate(content: &String) -> Result<Tree, Errors> {
    let mut parser = Parser::new(&content);

    parser.parse()
}

/// Prints the abstract syntax tree (AST) to the standard output with color-coding.
///
/// # Arguments
///
/// * `ast` - The abstract syntax tree to be printed.
///
/// # Returns
///
/// * `io::Result<()>` - Ok(()) if printing is successful, Err(io::Error) otherwise.
pub fn print_ast(ast: Tree) -> io::Result<()> {
    io::stdout().execute(SetForegroundColor(Color::Green))?;
    dbg!(ast.clone());
    io::stdout().execute(SetForegroundColor(Color::Blue))?;
    print_tree(&ast);
    io::stdout().execute(ResetColor)?;
    Ok(())
}

/// Prints errors in the abstract syntax tree (AST) to the standard output with color-coding.
///
/// # Arguments
///
/// * `ast` - The abstract syntax tree containing errors to be printed.
///
/// # Returns
///
/// * `io::Result<()>` - Ok(()) if printing is successful, Err(io::Error) otherwise.
pub fn print_error(errors: Errors) -> io::Result<()> {
    io::stdout().execute(SetForegroundColor(Color::Red))?;
    dbg!(errors.clone());
    io::stdout().execute(ResetColor)?;
    Ok(())
}

/// Prints the nodes of the abstract syntax tree (AST) in a tree-like structure.
///
/// # Arguments
///
/// * `tree` - The abstract syntax tree to be printed.
pub fn print_tree(tree: &Tree) {
    let mut indent = Vec::new();
    for (i, node) in tree.iter().enumerate() {
        let last = i == tree.len() - 1;
        print_node(node, &mut indent, last);
        println!();
    }

    fn print_node(node: &Node, indent: &mut Vec<&str>, last: bool) {
        match &**node {
            ASTNode::StringType => {
                if !indent.is_empty() {
                    for i in 0..indent.len() {
                        print!("{}", indent[i]);
                    }

                    if last {
                        print!("└───");
                    } else {
                        print!("├───");
                    }
                }
                println!("str");
            }

            ASTNode::BooleanType => {
                if !indent.is_empty() {
                    for i in 0..indent.len() {
                        print!("{}", indent[i]);
                    }

                    if last {
                        print!("└───");
                    } else {
                        print!("├───");
                    }
                }
                println!("bool");
            }

            ASTNode::NumberType => {
                if !indent.is_empty() {
                    for i in 0..indent.len() {
                        print!("{}", indent[i]);
                    }

                    if last {
                        print!("└───");
                    } else {
                        print!("├───");
                    }
                }
                println!("num");
            }

            ASTNode::BooleanLiteral(value) => {
                if !indent.is_empty() {
                    for i in 0..indent.len() {
                        print!("{}", indent[i]);
                    }

                    if last {
                        print!("└───");
                    } else {
                        print!("├───");
                    }
                }
                println!("{}", value);
            }

            ASTNode::StringLiteral(value)
            | ASTNode::NumberLiteral(value)
            | ASTNode::Identifier(value)
            | ASTNode::Operator(value) => {
                if !indent.is_empty() {
                    for i in 0..indent.len() {
                        print!("{}", indent[i]);
                    }

                    if last {
                        print!("└───");
                    } else {
                        print!("├───");
                    }
                }
                println!("{}", value);
            }

            ASTNode::Type(value) => {
                if !indent.is_empty() {
                    for i in 0..indent.len() {
                        print!("{}", indent[i]);
                    }

                    if last {
                        print!("└───");
                    } else {
                        print!("├───");
                    }
                }
                println!("[Type]");
                match value {
                    Some(value) => print_node(value, indent, false),
                    None => {}
                };
            }

            ASTNode::Return(value) => {
                if !indent.is_empty() {
                    for i in 0..indent.len() {
                        print!("{}", indent[i]);
                    }

                    if last {
                        print!("└───");
                    } else {
                        print!("├───");
                    }
                }
                println!("[Return]");
                match value {
                    Some(value) => print_node(value, indent, false),
                    None => {}
                };
            }

            ASTNode::UnaryExpression(op, expr) => {
                if !indent.is_empty() {
                    for i in 0..indent.len() {
                        print!("{}", indent[i]);
                    }

                    if last {
                        print!("└───");
                    } else {
                        print!("├───");
                    }
                }
                println!("{}{}", op, expr);
            }

            ASTNode::BinaryExpression(left, op, right) => {
                if !indent.is_empty() {
                    for i in 0..indent.len() {
                        print!("{}", indent[i]);
                    }

                    if last {
                        print!("└───");
                    } else {
                        print!("├───");
                    }
                }
                println!("{} {} {}", left, op, right);
            }

            ASTNode::VariableDeclaration(name, t) => {
                println!("[Variable Declaration]");

                print_node(name, indent, false);
                print_node(t, indent, true);
            }

            ASTNode::VariableDefinition(name, t, expr) => {
                println!("[Variable Definition]");

                print_node(name, indent, false);
                print_node(t, indent, false);
                print_node(expr, indent, true);
            }

            ASTNode::FunctionDefinition(id, params, ret, body) => {
                println!("[Function Definition]");

                print_node(id, indent, false);
                print_node(params, indent, false);
                print_node(ret, indent, false);
                print_node(body, indent, true);
            }

            ASTNode::Parameters(children) => {
                println!("[Parameters]");

                let len = children.len();
                for (i, child) in children.iter().enumerate() {
                    let next_last = last && i == len - 1;
                    let indent_next = if next_last { "    " } else { "│   " };
                    indent.push(indent_next);
                    print_node(child, indent, next_last);
                    indent.pop();
                }
            }

            ASTNode::Block(children) => {
                println!("[Block]");

                let len = children.len();
                for (i, child) in children.iter().enumerate() {
                    let next_last = last && i == len - 1;
                    let indent_next = if next_last { "    " } else { "│   " };
                    indent.push(indent_next);
                    print_node(child, indent, next_last);
                    indent.pop();
                }
            }

            ASTNode::FunctionCall(name, arguments) => {
                println!("[Function Call]");

                print_node(name, indent, false);
                print_node(arguments, indent, true);
            }

            ASTNode::Arguments(children) => {
                println!("[Arguments]");

                let len = children.len();
                for (i, child) in children.iter().enumerate() {
                    let next_last = last && i == len - 1;
                    let indent_next = if next_last { "    " } else { "│   " };
                    indent.push(indent_next);
                    print_node(child, indent, next_last);
                    indent.pop();
                }
            }

            ASTNode::If(condition, affermative, negative) => {
                println!("[If]");

                print_node(condition, indent, false);
                print_node(affermative, indent, false);
                print_node(negative, indent, true);
            }

            ASTNode::While(condition, body) => {
                println!("[While]");

                print_node(condition, indent, false);
                print_node(body, indent, true);
            }

            ASTNode::Array(children) => {
                println!("[Array]");

                let len = children.len();
                for (i, child) in children.iter().enumerate() {
                    let next_last = last && i == len - 1;
                    let indent_next = if next_last { "    " } else { "│   " };
                    indent.push(indent_next);
                    print_node(child, indent, next_last);
                    indent.pop();
                }
            }

            ASTNode::ParenDelimiter => todo!(),
            ASTNode::BraceDelimiter => todo!(),
            ASTNode::BracketDelimiter => todo!(),
            ASTNode::Separator => todo!(),
        }
    }

    // fn print_message(message: &str, indent: &mut Vec<&str>, last: bool) {
    //     if !indent.is_empty() {
    //         for i in 0..indent.len() {
    //             print!("{}", indent[i]);
    //         }

    //         if last {
    //             print!("└───");
    //         } else {
    //             print!("├───");
    //         }
    //     }
    //     println!("[{}]", message);
    // }
}

use std::io;

use crate::hash::ast::Node;

use self::{ast::ASTError, ast::ASTNode, parser::Parser};

use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod tokens;

type Tree = Vec<Box<ASTNode>>;

pub fn validate(content: &String) -> Result<Tree, Vec<Box<ASTError>>> {
    let mut parser = Parser::new(&content);

    parser.parse()
}

pub fn print_ast(ast: Vec<Box<ast::ASTNode>>) -> io::Result<()> {
    // io::stdout().execute(SetForegroundColor(Color::Green))?;
    // println!("{:?}", ast);
    io::stdout().execute(SetForegroundColor(Color::Blue))?;
    print_tree(&ast);
    io::stdout().execute(ResetColor)?;
    Ok(())
}

pub fn print_error(ast: Vec<Box<ast::ASTError>>) -> io::Result<()> {
    io::stdout().execute(SetForegroundColor(Color::Red))?;
    println!("{:?}", ast);
    io::stdout().execute(ResetColor)?;
    Ok(())
}

pub fn print_tree(tree: &Tree) {
    let mut indent = Vec::new();
    for (i, node) in tree.iter().enumerate() {
        let last = i == tree.len() - 1;
        print_node(node, &mut indent, last);
        println!();
    }

    fn print_node(node: &Node, indent: &mut Vec<&str>, last: bool) {
        match &**node {
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
            ASTNode::VariableDefinition(name, expr) => {
                println!("[Variable Definition]");
                print_node(name, indent, false);
                print_node(expr, indent, true);
            }
            ASTNode::FunctionDefinition(id, params, body) => {
                println!("[Function Definition]");
                print_node(id, indent, false);
                print_node(params, indent, false);
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
            _ => {}
        }
    }
}

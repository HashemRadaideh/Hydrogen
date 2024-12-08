use std::io;

use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};

use super::ast::{ASTNode, Errors, Node, Tree};

/// Prints the abstract syntax tree (AST) to the standard output with color-coding.
///
/// # Arguments
///
/// * `ast` - The abstract syntax tree to be printed.
///
/// # Returns
///
/// * `io::Result<()>` - Ok(()) if printing is successful, Err(io::Error) otherwise.
pub fn print_ast(ast: &Tree) -> io::Result<()> {
    io::stdout().execute(SetForegroundColor(Color::Green))?;
    dbg!(ast.clone());
    io::stdout().execute(SetForegroundColor(Color::Blue))?;
    print_tree(ast);
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
                    (0..indent.len()).for_each(|i| {
                        print!("{}", indent[i]);
                    });

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
                    (0..indent.len()).for_each(|i| {
                        print!("{}", indent[i]);
                    });

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
                    (0..indent.len()).for_each(|i| {
                        print!("{}", indent[i]);
                    });

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
                    (0..indent.len()).for_each(|i| {
                        print!("{}", indent[i]);
                    });

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
                    (0..indent.len()).for_each(|i| {
                        print!("{}", indent[i]);
                    });

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
                    (0..indent.len()).for_each(|i| {
                        print!("{}", indent[i]);
                    });

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
                    (0..indent.len()).for_each(|i| {
                        print!("{}", indent[i]);
                    });

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
                    (0..indent.len()).for_each(|i| {
                        print!("{}", indent[i]);
                    });

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
                    (0..indent.len()).for_each(|i| {
                        print!("{}", indent[i]);
                    });

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
            ASTNode::End => todo!(),
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

#[cfg(test)]
mod test {
    use crate::hash::{ast::Nodes, parser::Parser};

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

        let mut results: Nodes = Vec::new();
        let mut errors: Errors = Vec::new();

        loop {
            match parser.parse() {
                Ok(node) => match *node {
                    ASTNode::End => break,
                    _ => {
                        results.push(node);
                    }
                },
                Err(error) => {
                    errors.push(error);
                }
            }
        }

        if errors.is_empty() {
            let _ = print_ast(&results);
            print_tree(&results);
        } else {
            let _ = print_error(errors);
        }

        // TODO: Add more specific assertions based on your expected AST structure
        // For example, you can assert the structure of the AST, the types of nodes, etc.
    }
}

use std::io;

use self::{ast::Error, ast::Node, parser::Parser};

use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod tokens;

pub fn validate(content: &String) -> Result<Vec<Box<Node>>, Vec<Box<Error>>> {
    let mut parser = Parser::new(&content);

    parser.parse()
}

pub fn print_ast(ast: Vec<Box<ast::Node>>) -> io::Result<()> {
    io::stdout().execute(SetForegroundColor(Color::Green))?;
    println!("{:?}", ast);
    io::stdout().execute(ResetColor)?;
    Ok(())
}

pub fn print_error(ast: Vec<Box<ast::Error>>) -> io::Result<()> {
    io::stdout().execute(SetForegroundColor(Color::Red))?;
    println!("{:?}", ast);
    io::stdout().execute(ResetColor)?;
    Ok(())
}

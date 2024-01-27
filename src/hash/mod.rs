use self::{ast::Error, ast::Node, parser::Parser};

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod tokens;

pub fn validate(content: &String) -> Result<Vec<Box<Node>>, Vec<Box<Error>>> {
    let mut parser = Parser::new(&content);

    parser.parse()
}

pub fn exec(ast: Vec<Box<ast::Node>>) {
    println!("{:?}", ast);
}

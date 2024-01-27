use self::{ast::ASTree, parser::Parser};

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod tokens;

pub fn validate(content: &String) -> (Option<ASTree>, bool) {
    let mut parser = Parser::new(&content);

    let ast = parser.parse();

    match ast {
        Ok(tree) => (Some(tree), true),
        Err(_) => (None, false),
    }
}

pub fn exec(ast: ASTree) {
    println!("{:?}", ast);
}

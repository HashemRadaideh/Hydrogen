use self::{ast::ASTree, lexer::Lexer, parser::Parser};

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod tokens;

pub fn validate(content: &String) -> bool {
    let mut lexer = Lexer::new(&content);

    let mut parser = Parser::new(lexer.lex());

    let ast = parser.parse();

    print_ast(ast);

    true
}

fn print_ast(ast: ASTree) {
    println!("{:?}", ast);
}

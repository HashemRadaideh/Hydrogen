use super::ast::{ASTNode, Error, Errors, Node, Nodes};
use super::parser::Parser;
use super::print::{print_ast, print_error};

pub struct Evaluator<'a> {
    parser: Parser<'a>,
}

impl<'a> Evaluator<'a> {
    /// Creates a new Evaluator instance with the given program source code.
    pub fn new(program: &'a str) -> Self {
        Self {
            parser: Parser::new(program),
        }
    }

    /// Advances the parser and returns the next ast node.
    fn next(&mut self) -> Result<Node, Error> {
        self.parser.parse()
    }

    /// Validates the input content and returns the AST or errors.
    ///
    /// # Arguments
    ///
    /// * `content` - The content to be parsed and validated.
    ///
    /// # Returns
    ///
    /// * `Result<Tree, Errors>` - Ok(Tree) if parsing is successful, Err(Errors) if there are errors.
    fn validate(&mut self) -> bool {
        match self.next() {
            Ok(statement) => self.check_types(statement),
            Err(_) => false,
        }
    }

    fn check_types(&mut self, statement: Node) -> bool {
        println!("{:?}", statement);
        true
    }

    pub fn eval(&mut self) {
        let mut results: Nodes = Vec::new();
        let mut errors: Errors = Vec::new();

        loop {
            match self.next() {
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

        if errors.is_empty() && self.validate() {
            let _ = print_ast(&results);
        } else {
            let _ = print_error(errors);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{fs, path::Path};

    #[test]
    fn test_evaluator() {
        // Read and validate code from the specified script file.
        let path = fs::read_to_string(Path::new("test/hello.hy")).unwrap();
        let mut evaluator = Evaluator::new(&path);
        evaluator.eval();
    }
}

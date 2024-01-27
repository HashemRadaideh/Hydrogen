use std::fmt;

#[derive(Debug)]
pub enum ASTNode {
    Program(Vec<Box<ASTNode>>),
    // VariableDeclaration(String, Box<ASTNode>),
    // Assignment(String, Box<ASTNode>),
    Identifier(String),
    // Number(f64),
    // StringLiteral(String),
    // Boolean(bool),
}

#[derive(Debug)]
pub struct ASTree {
    pub root: Option<Box<ASTNode>>,
}

impl ASTree {
    pub fn new() -> Self {
        Self { root: None }
    }
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format(0))
    }
}

impl ASTNode {
    fn format(&self, level: usize) -> String {
        let indent = "  ".repeat(level);

        match self {
            ASTNode::Program(statements) => {
                let mut result = String::new();
                for statement in statements {
                    result.push_str(&statement.format(level + 1));
                }
                result
            }
            // ASTNode::VariableDeclaration(identifier, expression) => {
            //     format!(
            //         "{}[Variable Declaration] {} = {}\n",
            //         indent,
            //         identifier,
            //         expression.format(level + 1)
            //     )
            // }
            // ASTNode::Assignment(identifier, expression) => {
            //     format!(
            //         "{}[Assignment] {} = {}\n",
            //         indent,
            //         identifier,
            //         expression.format(level + 1)
            //     )
            // }
            ASTNode::Identifier(identifier) => format!("{}[Identifier] {}\n", indent, identifier),
            // ASTNode::Number(number) => format!("{}[Number] {}\n", indent, number),
            // ASTNode::StringLiteral(s) => format!("{}[String Literal] \"{}\"\n", indent, s),
            // ASTNode::Boolean(b) => format!("{}[Boolean] {}\n", indent, b),
        }
    }
}

impl fmt::Display for ASTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(root) = &self.root {
            write!(f, "{}", root.format(0))
        } else {
            write!(f, "Empty AST\n")
        }
    }
}

#[cfg(test)]
mod tests {}

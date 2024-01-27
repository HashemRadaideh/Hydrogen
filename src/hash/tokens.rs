#[derive(Debug, Clone)]
pub enum TokenType {
    String,
    Integer,
    Unknown,
    Whitespace,
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    pub token: TokenType,
}

impl Token {
    pub fn new(value: String, token: TokenType) -> Self {
        Self { value, token }
    }
}

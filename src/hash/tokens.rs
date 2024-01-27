#[derive(Debug, Clone)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

#[derive(Debug, Clone)]
pub enum Token {
    LeftParenthesis(Position),
    RightParenthesis(Position),
    LeftBrace(Position),
    RightBrace(Position),
    LeftBracket(Position),
    RightBracket(Position),
    Assignment(Position),
    Plus(Position),
    Minus(Position),
    Asterisk(Position),
    Slash(Position),
    Ampersand(Position),
    DollarSign(Position),
    Hash(Position),
    ExplinationMark(Position),
    QuestionMark(Position),
    EscapeCode(Position, char),
    Identifier(Position, String),
    String(Position, String),
    Boolean(Position, bool),
    Integer(Position, i32),
    Float(Position, f32),
    Keyword(Position, String),
    Unknown(Position, String),
    Whitespace,
    EOF(Position),
}
